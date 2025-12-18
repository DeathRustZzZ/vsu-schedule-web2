package com.vsuscheduleweb.services;


import com.vsuscheduleweb.DTO.GroupResponse;
import com.vsuscheduleweb.DTO.ListGroupResponse;
import com.vsuscheduleweb.Exceptions.Errors.AppError;
import com.vsuscheduleweb.Exceptions.FileException;
import com.vsuscheduleweb.Exceptions.FileIsEmptyException;
import com.vsuscheduleweb.entity.Group;
import com.vsuscheduleweb.entity.Lesson;
import com.vsuscheduleweb.entity.Subgroup;
import com.vsuscheduleweb.entity.Teacher;
import com.vsuscheduleweb.mapper.GroupMapper;
import com.vsuscheduleweb.parser.Parser;
import com.vsuscheduleweb.Exceptions.ParserException;
import com.vsuscheduleweb.repositories.GroupRepository;
import com.vsuscheduleweb.repositories.LessonRepository;
import com.vsuscheduleweb.repositories.TeacherRepository;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.apache.xmlbeans.impl.piccolo.io.FileFormatException;
import org.springframework.dao.IncorrectResultSizeDataAccessException;
import org.springframework.http.HttpEntity;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;

import org.springframework.stereotype.Service;
import org.springframework.web.multipart.MultipartFile;

import java.util.List;
import java.io.*;
import java.util.Locale;
import java.util.Optional;

@Service
@RequiredArgsConstructor
@Slf4j
public class ScheduleService {

    private final Parser parser;

    private final TeacherRepository teacherRepository;

    private final GroupRepository groupRepository;

    private final LessonRepository lessonRepository;

    public void uploadSchedule(MultipartFile multipartFile, String facult) {
        lessonRepository.deleteAllWhereFacultEquals(facult);
        File file = saveFile(multipartFile,facult);
        parser.parse(file, facult);
        List<Group> groups = parser.getGroups();
        List<Lesson> lessons = parser.getLessons();
        List<Teacher> teachers = parser.getTeachers();
        for (Lesson lesson : lessons) {
            lesson.setFacult(facult);
            lesson.setTeacherId(-1);
        }
        processTeachers(teachers);
        processGroups(groups);
       
    }

    private File saveFile(MultipartFile multipartFile, String facult) {
        String path = System.getProperty("user.dir") + "\\src\\main\\temp";
        lessonRepository.deleteAllWhereFacultEquals(facult);
        if (!multipartFile.isEmpty()) {
            File file = new File(path + "\\" + multipartFile.getOriginalFilename());
            if (file.exists()) file.delete();
            file = new File(path + "\\" + multipartFile.getOriginalFilename());
            try{
                FileOutputStream fileOutputStream = new FileOutputStream(file);
                fileOutputStream.write(multipartFile.getBytes());
                fileOutputStream.close();
                return file;
            }catch (Exception e){
                throw new FileException(e.getMessage());
            }
        } else
            throw new FileIsEmptyException("file cannot be empty.");
    }

    private void processTeachers(List<Teacher> teachers){
        teachers.forEach(teacher -> {
            try {
                Optional<Teacher> opt_teacher = teacherRepository.findByInitialsAndLastname(teacher.getInitials(),
                        teacher.getLastname().toUpperCase(Locale.ROOT));
                if (opt_teacher.isPresent()) {
                    Teacher teacherDb = opt_teacher.get();
                    teacher.getLessons().forEach(lesson -> {
                        lesson.setTeacherId(teacherDb.getId());
                    });
                    teacherDb.setLessons(teacher.getLessons());
                    teacherRepository.save(teacherDb);
                }
            } catch (IncorrectResultSizeDataAccessException e) {
                log.error(e.getMessage());
            }
        });

    }

    private void processGroups(List<Group> groups){
        for (Group group : groups) {
            group.setId(group.getId().replace('/','.'));
            for (int j = 0; j < group.getCommonLessons().size(); j++) {
                Lesson lesson = group.getCommonLessons().get(j);
                lesson.setGroupId(group.getId());
            }
            for (int j = 0; j < group.getSubgroups().size(); j++) {
                Subgroup subgroup = group.getSubgroups().get(j);
                subgroup.setGroupId(group.getId());
                for (int k = 0; k < subgroup.getLessons().size(); k++) {
                    Lesson lesson = subgroup.getLessons().get(k);
                    lesson.setSubgroupId(subgroup.getId());
                }
            }
        }
        groupRepository.saveAll(groups);
    }
}

