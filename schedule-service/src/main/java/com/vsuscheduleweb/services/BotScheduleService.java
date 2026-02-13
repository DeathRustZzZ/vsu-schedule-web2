package com.vsuscheduleweb.services;

import com.vsuscheduleweb.DTO.ListLessonResponse;
import com.vsuscheduleweb.mapper.LessonMapper;
import com.vsuscheduleweb.repositories.LessonRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;

import java.time.LocalDate;
import java.time.format.DateTimeFormatter;
import java.time.format.DateTimeParseException;
import java.time.temporal.ChronoUnit;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Locale;
import java.util.Optional;
import java.util.Set;
import java.util.Map;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

@Service
@RequiredArgsConstructor
public class BotScheduleService {
    private final BotScheduleCacheService cacheService;
    private final LessonRepository lessonRepository;
    private final LessonMapper lessonMapper;
    private static final long MAX_RANGE_DAYS = 7;

    public ListLessonResponse getSchedule(String faculty, String groupId, String subgroupId, String weekDay) {
        return cacheService
                .get(faculty, groupId, subgroupId, weekDay)
                .orElseGet(() -> {
                    var lessons = lessonRepository
                            .findByFacultyAndGroupOrSubgroupAndWeekDay(faculty, groupId, subgroupId, weekDay)
                            .stream()
                            .map(lessonMapper::entityToResponse)
                            .toList();
                    ListLessonResponse response = new ListLessonResponse(lessons);
                    cacheService.put(faculty, groupId, subgroupId, weekDay, response);
                    return response;
                });
    }

    public ListLessonResponse getScheduleByDate(String faculty, String groupId, String subgroupId, String date) {
        Optional<LocalDate> target = parseDate(date);
        Set<String> candidateKeys = buildDateKeys(date, target);

        var lessons = lessonRepository
                .findByFacultyAndGroupOrSubgroup(faculty, groupId, subgroupId)
                .stream()
                .filter(lesson -> matchesDate(lesson.getDate(), date, target, candidateKeys))
                .map(lessonMapper::entityToResponse)
                .toList();

        return new ListLessonResponse(lessons);
    }

    public ListLessonResponse getScheduleByDateRange(
            String faculty,
            String groupId,
            String subgroupId,
            String start,
            String end
    ) {
        Optional<LocalDate> startDateOpt = parseDate(start);
        Optional<LocalDate> endDateOpt = parseDate(end);

        if (startDateOpt.isEmpty()) {
            return getScheduleByDate(faculty, groupId, subgroupId, start);
        }

        LocalDate startDate = startDateOpt.get();
        LocalDate endDate = endDateOpt.orElse(startDate);
        if (endDate.isBefore(startDate)) {
            LocalDate tmp = startDate;
            startDate = endDate;
            endDate = tmp;
        }
        long days = ChronoUnit.DAYS.between(startDate, endDate) + 1;
        if (days > MAX_RANGE_DAYS) {
            endDate = startDate.plusDays(MAX_RANGE_DAYS - 1);
        }

        LocalDate finalStartDate = startDate;
        LocalDate finalEndDate = endDate;
        var lessons = lessonRepository
                .findByFacultyAndGroupOrSubgroup(faculty, groupId, subgroupId)
                .stream()
                .filter(lesson -> inDateRange(lesson.getDate(), finalStartDate, finalEndDate))
                .map(lessonMapper::entityToResponse)
                .toList();

        return new ListLessonResponse(lessons);
    }

    private static boolean matchesDate(
            String lessonDate,
            String rawInput,
            Optional<LocalDate> target,
            Set<String> candidateKeys
    ) {
        if (lessonDate == null || lessonDate.isBlank()) {
            return false;
        }
        String value = lessonDate.trim();
        if (candidateKeys.contains(value)) {
            return true;
        }
        Optional<LocalDate> parsedLesson = parseDate(value);
        if (parsedLesson.isPresent() && target.isPresent()) {
            return parsedLesson.get().isEqual(target.get());
        }
        if (target.isPresent() && isShortDate(value)) {
            return shortDateEquals(value, target.get());
        }
        return value.equals(rawInput);
    }

    private static boolean inDateRange(String lessonDate, LocalDate start, LocalDate end) {
        if (lessonDate == null || lessonDate.isBlank()) {
            return false;
        }
        String value = lessonDate.trim();
        Optional<LocalDate> parsed = parseDate(value);
        if (parsed.isPresent()) {
            LocalDate date = parsed.get();
            return !date.isBefore(start) && !date.isAfter(end);
        }
        if (isShortDate(value)) {
            LocalDate projected = projectShortDate(value, start.getYear());
            if (projected != null) {
                return !projected.isBefore(start) && !projected.isAfter(end);
            }
        }
        return false;
    }

    private static Set<String> buildDateKeys(String rawInput, Optional<LocalDate> target) {
        Set<String> keys = new HashSet<>();
        if (rawInput != null) {
            keys.add(rawInput.trim());
        }
        if (target.isPresent()) {
            LocalDate date = target.get();
            keys.add(date.format(DateTimeFormatter.ofPattern("dd.MM.yyyy", Locale.ROOT)));
            keys.add(date.format(DateTimeFormatter.ofPattern("dd.MM.yy", Locale.ROOT)));
            keys.add(date.format(DateTimeFormatter.ofPattern("yyyy-MM-dd", Locale.ROOT)));
            keys.add(date.format(DateTimeFormatter.ofPattern("dd/MM/yyyy", Locale.ROOT)));
            keys.add(date.format(DateTimeFormatter.ofPattern("dd/MM/yy", Locale.ROOT)));
            keys.add(date.format(DateTimeFormatter.ofPattern("dd.MM", Locale.ROOT)));
            keys.add(date.format(DateTimeFormatter.ofPattern("dd/MM", Locale.ROOT)));
            DateTimeFormatter ru = DateTimeFormatter.ofPattern("d MMMM yyyy 'г.'", new Locale("ru", "RU"));
            DateTimeFormatter ruNoDot = DateTimeFormatter.ofPattern("d MMMM yyyy 'г'", new Locale("ru", "RU"));
            DateTimeFormatter ruPlain = DateTimeFormatter.ofPattern("d MMMM yyyy", new Locale("ru", "RU"));
            keys.add(date.format(ru));
            keys.add(date.format(ruNoDot));
            keys.add(date.format(ruPlain));
        }
        return keys;
    }

    private static Optional<LocalDate> parseDate(String value) {
        if (value == null) {
            return Optional.empty();
        }
        String raw = value.trim();
        if (raw.isEmpty()) {
            return Optional.empty();
        }
        for (DateTimeFormatter formatter : dateFormats()) {
            try {
                return Optional.of(LocalDate.parse(raw, formatter));
            } catch (DateTimeParseException ignored) {
            }
        }
        Optional<LocalDate> russian = parseRussianDate(raw);
        if (russian.isPresent()) {
            return russian;
        }
        return Optional.empty();
    }

    private static List<DateTimeFormatter> dateFormats() {
        List<DateTimeFormatter> formats = new ArrayList<>();
        formats.add(DateTimeFormatter.ofPattern("yyyy-MM-dd", Locale.ROOT));
        formats.add(DateTimeFormatter.ofPattern("dd.MM.yyyy", Locale.ROOT));
        formats.add(DateTimeFormatter.ofPattern("dd.MM.yy", Locale.ROOT));
        formats.add(DateTimeFormatter.ofPattern("dd/MM/yyyy", Locale.ROOT));
        formats.add(DateTimeFormatter.ofPattern("dd/MM/yy", Locale.ROOT));
        return formats;
    }

    private static Optional<LocalDate> parseRussianDate(String raw) {
        Matcher matcher = RUS_DATE_PATTERN.matcher(raw.toLowerCase(Locale.ROOT));
        if (!matcher.matches()) {
            return Optional.empty();
        }
        try {
            int day = Integer.parseInt(matcher.group(1));
            String monthName = matcher.group(2);
            int year = Integer.parseInt(matcher.group(3));
            Integer month = RUS_MONTHS.get(monthName);
            if (month == null) {
                return Optional.empty();
            }
            return Optional.of(LocalDate.of(year, month, day));
        } catch (RuntimeException ex) {
            return Optional.empty();
        }
    }

    private static boolean isShortDate(String value) {
        return value.matches("\\d{1,2}[./]\\d{1,2}");
    }

    private static boolean shortDateEquals(String shortDate, LocalDate target) {
        LocalDate projected = projectShortDate(shortDate, target.getYear());
        return projected != null && projected.isEqual(target);
    }

    private static LocalDate projectShortDate(String shortDate, int year) {
        String normalized = shortDate.replace('/', '.').trim();
        String[] parts = normalized.split("\\.");
        if (parts.length != 2) {
            return null;
        }
        try {
            int day = Integer.parseInt(parts[0]);
            int month = Integer.parseInt(parts[1]);
            return LocalDate.of(year, month, day);
        } catch (RuntimeException ex) {
            return null;
        }
    }

    private static final Pattern RUS_DATE_PATTERN = Pattern.compile(
            "(\\d{1,2})\\s+([а-яё]+)\\s+(\\d{4})(?:\\s*г\\.?)*",
            Pattern.CASE_INSENSITIVE
    );

    private static final Map<String, Integer> RUS_MONTHS = buildRusMonths();

    private static Map<String, Integer> buildRusMonths() {
        Map<String, Integer> map = new HashMap<>();
        map.put("января", 1);
        map.put("февраля", 2);
        map.put("марта", 3);
        map.put("апреля", 4);
        map.put("мая", 5);
        map.put("июня", 6);
        map.put("июля", 7);
        map.put("августа", 8);
        map.put("сентября", 9);
        map.put("октября", 10);
        map.put("ноября", 11);
        map.put("декабря", 12);
        return map;
    }
}
