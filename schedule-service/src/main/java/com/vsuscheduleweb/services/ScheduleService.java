package com.vsuscheduleweb.services;


import com.vsuscheduleweb.Exceptions.FileException;
import com.vsuscheduleweb.Exceptions.FileIsEmptyException;
import com.vsuscheduleweb.entity.Group;
import com.vsuscheduleweb.entity.Lesson;
import com.vsuscheduleweb.entity.Subgroup;
import com.vsuscheduleweb.entity.Teacher;
import com.vsuscheduleweb.parser.Parser;
import com.vsuscheduleweb.repositories.GroupRepository;
import com.vsuscheduleweb.repositories.LessonRepository;
import com.vsuscheduleweb.repositories.TeacherRepository;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.dao.IncorrectResultSizeDataAccessException;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;
import org.springframework.web.multipart.MultipartFile;
import java.util.List;
import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.LinkedHashMap;
import java.util.Locale;
import java.util.Map;
import java.util.Optional;
import java.util.concurrent.atomic.AtomicInteger;


@Service
@RequiredArgsConstructor
@Slf4j
public class ScheduleService {

    private final Parser parser;

    private final TeacherRepository teacherRepository;

    private final GroupRepository groupRepository;

    private final LessonRepository lessonRepository;

    private final BotScheduleCacheService botScheduleCacheService;

    @Transactional
    public void uploadSchedule(MultipartFile multipartFile, String faculty) {
        String normalizedFaculty = normalizeFaculty(faculty);
        File file = saveFile(multipartFile);
        parser.parse(file, normalizedFaculty);
        List<Group> groups = parser.getGroups();
        List<Lesson> lessons = parser.getLessons();
        List<Teacher> teachers = parser.getTeachers();
        lessonRepository.deleteAllWhereFacultyEquals(normalizedFaculty);
        for (Lesson lesson : lessons) {
            lesson.setFaculty(normalizedFaculty);
            lesson.setTeacherId(-1);
        }
        processTeachers(teachers);
        processGroups(groups, normalizedFaculty);
        botScheduleCacheService.rebuildForFaculty(normalizedFaculty, groups, lessons);

    }

    private File saveFile(MultipartFile multipartFile) {
        if (multipartFile.isEmpty()) {
            throw new FileIsEmptyException("file cannot be empty.");
        }
        Path baseDir = Paths.get(System.getProperty("user.dir"), "schedule-service", "src", "main", "temp");
        try {
            Files.createDirectories(baseDir);
            String originalName = multipartFile.getOriginalFilename();
            String safeName = originalName == null || originalName.isBlank() ? "schedule.xlsx" : originalName;
            Path target = baseDir.resolve(safeName);
            multipartFile.transferTo(target);
            return target.toFile();
        } catch (IOException e) {
            throw new FileException(e.getMessage());
        }
    }

    private void processTeachers(List<Teacher> teachers) {
        Map<String, Teacher> mergedTeachers = new LinkedHashMap<>();
        for (Teacher teacher : teachers) {
            String initials = normalizeInitials(teacher.getInitials());
            String lastname = normalizeLastname(teacher.getLastname());
            if (initials.isBlank() && lastname.isBlank()) {
                continue;
            }
            String key = (lastname + "|" + initials).toLowerCase(Locale.ROOT);
            Teacher existing = mergedTeachers.get(key);
            if (existing == null) {
                Teacher copy = new Teacher();
                copy.setLastname(lastname);
                copy.setInitials(initials);
                copy.setQualification(teacher.getQualification());
                copy.setFirstname(teacher.getFirstname());
                copy.setSurname(teacher.getSurname());
                copy.setFullname(teacher.getFullname());
                copy.getLessons().addAll(teacher.getLessons());
                mergedTeachers.put(key, copy);
            } else {
                existing.getLessons().addAll(teacher.getLessons());
            }
        }

        Integer maxId = teacherRepository.findMaxId();
        AtomicInteger nextId = new AtomicInteger((maxId == null ? -1 : maxId) + 1);

        mergedTeachers.values().forEach(teacher -> {
            try {
                String initials = normalizeInitials(teacher.getInitials());
                String lastname = normalizeLastname(teacher.getLastname());
                Optional<Teacher> opt_teacher = teacherRepository.findByInitialsAndLastnameIgnoreCase(
                        initials,
                        lastname
                );
                if (opt_teacher.isPresent()) {
                    Teacher teacherDb = opt_teacher.get();
                    teacher.getLessons().forEach(lesson -> lesson.setTeacherId(teacherDb.getId()));
                    teacherDb.setLessons(teacher.getLessons());
                    teacherRepository.save(teacherDb);
                } else {
                    teacher.setId(nextId.getAndIncrement());
                    teacher.setLastname(lastname);
                    teacher.setInitials(initials);
                    teacher.setFirstname(defaultString(teacher.getFirstname()));
                    teacher.setSurname(defaultString(teacher.getSurname()));
                    if (teacher.getFullname() == null || teacher.getFullname().isBlank()) {
                        teacher.setFullname((lastname + " " + initials).trim());
                    }
                    teacher.getLessons().forEach(lesson -> lesson.setTeacherId(teacher.getId()));
                    teacherRepository.save(teacher);
                }
            } catch (IncorrectResultSizeDataAccessException e) {
                log.error(e.getMessage());
            }
        });
    }

    private String defaultString(String value) {
        return value == null ? "" : value;
    }

    private String normalizeInitials(String initials) {
        if (initials == null) {
            return "";
        }
        String normalized = initials.replaceAll("\\s+", " ").trim();
        if (normalized.length() > 20) {
            log.warn("Initials too long ({}), truncating: '{}'", normalized.length(), normalized);
            normalized = normalized.substring(0, 20);
        }
        return normalized;
    }

    private String normalizeLastname(String lastname) {
        if (lastname == null) {
            return "";
        }
        return lastname.trim();
    }

    private void processGroups(List<Group> groups, String faculty) {
        for (Group group : groups) {
            group.setFaculty(faculty);
            for (int j = 0; j < group.getCommonLessons().size(); j++) {
                Lesson lesson = group.getCommonLessons().get(j);
                lesson.setGroupId(group.getId());
            }
            for (int j = 0; j < group.getSubgroups().size(); j++) {
                Subgroup subgroup = group.getSubgroups().get(j);
                subgroup.setGroupId(group.getId());
                subgroup.setFaculty(faculty);
                for (int k = 0; k < subgroup.getLessons().size(); k++) {
                    Lesson lesson = subgroup.getLessons().get(k);
                    lesson.setSubgroupId(subgroup.getId());
                    lesson.setGroupId(group.getId());
                }
            }
        }
        groupRepository.saveAll(groups);
    }

    private String normalizeFaculty(String faculty) {
        if (faculty == null) {
            return "";
        }
        String normalized = faculty.trim().toLowerCase(Locale.ROOT);
        if (normalized.equals("фмиит") || normalized.equals("fmiit")) {
            return "ФМиИТ";
        }
        if (normalized.contains("математики") && normalized.contains("информационных")) {
            return "ФМиИТ";
        }
        if (normalized.equals("педфак") || normalized.contains("педагог")) {
            return "Педфак";
        }
        if (normalized.equals("юрфак") || normalized.contains("юрид")) {
            return "ЮрФак";
        }
        if (normalized.contains("хим") || normalized.contains("био")) {
            return "ХИМБИО";
        }
        if (normalized.contains("физк") || normalized.contains("спорт")) {
            return "Физкультура";
        }
        return faculty.trim();
    }
}
