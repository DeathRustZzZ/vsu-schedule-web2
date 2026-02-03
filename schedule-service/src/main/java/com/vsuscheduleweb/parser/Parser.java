package com.vsuscheduleweb.parser;

import com.vsuscheduleweb.Exceptions.ParserException;
import com.vsuscheduleweb.entity.Group;
import com.vsuscheduleweb.entity.Lesson;
import com.vsuscheduleweb.entity.Subgroup;
import com.vsuscheduleweb.entity.Teacher;
import lombok.NoArgsConstructor;
import org.apache.poi.ss.usermodel.Cell;
import org.apache.poi.ss.usermodel.DataFormatter;
import org.apache.poi.ss.usermodel.Row;
import org.apache.poi.ss.usermodel.Sheet;
import org.apache.poi.ss.usermodel.Workbook;
import org.apache.poi.ss.usermodel.WorkbookFactory;
import org.apache.poi.ss.util.CellRangeAddress;
import org.springframework.stereotype.Component;

import java.io.File;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Locale;
import java.util.Map;
import java.util.Set;
import java.util.UUID;
import java.util.regex.Matcher;
import java.util.regex.Pattern;


@NoArgsConstructor
@Component
public class Parser {

    private static final Set<String> DAY_NAMES = Set.of(
            "\u041f\u043e\u043d\u0435\u0434\u0435\u043b\u044c\u043d\u0438\u043a",
            "\u0412\u0442\u043e\u0440\u043d\u0438\u043a",
            "\u0421\u0440\u0435\u0434\u0430",
            "\u0427\u0435\u0442\u0432\u0435\u0440\u0433",
            "\u041f\u044f\u0442\u043d\u0438\u0446\u0430",
            "\u0421\u0443\u0431\u0431\u043e\u0442\u0430",
            "\u0412\u043e\u0441\u043a\u0440\u0435\u0441\u0435\u043d\u044c\u0435"
    );

    private static final Pattern SUBGROUP_PATTERN = Pattern.compile(".+_\\d+");
    private static final Pattern TIME_RANGE_PATTERN =
            Pattern.compile("\\(?\\s*(\\d{1,2}[:.]\\d{2})\\s*-\\s*(\\d{1,2}[:.]\\d{2})\\s*\\)?");
    private static final Pattern TIME_ONLY_PATTERN = Pattern.compile("\\b(\\d{1,2}[:.]\\d{2})\\b");
    private static final Pattern QUALIFICATION_PATTERN = Pattern.compile("\\(([^)]*)\\)\\s*$");
    private static final Pattern DATE_PATTERN = Pattern.compile("(\\d{1,2}[./]\\d{1,2}[./]\\d{2,4})");
    private static final Pattern TEACHER_LINE_PATTERN = Pattern.compile("([А-ЯЁ][а-яё]+)\\s+[А-ЯЁ]\\.?\\s*[А-ЯЁ]\\.?"); 

    private List<Teacher> teachers = new ArrayList<>();
    private List<Group> groups = new ArrayList<>();
    private List<Lesson> lessons = new ArrayList<>();

    public List<Teacher> getTeachers() {
        for (Teacher teacher : teachers) {
            teacher.getLessons().removeIf(lesson -> lesson.getName().equals(""));
        }
        return teachers;
    }

    public List<Group> getGroups() {
        for (Group group : groups) {
            group.getCommonLessons().removeIf(lesson -> lesson.getName().equals(""));
            for (Subgroup subgroup : group.getSubgroups()) {
                subgroup.getLessons().removeIf(lesson -> lesson.getName().equals(""));
            }
        }
        return groups;
    }

    public List<Lesson> getLessons() {
        return lessons;
    }

    public void parse(File xlsxFile, String faculty) throws ParserException {
        groups = new ArrayList<>();
        teachers = new ArrayList<>();
        lessons = new ArrayList<>();

        Workbook wb = readWorkbook(xlsxFile);
        try {
            Sheet sheet = wb.getSheetAt(0);
            SheetGrid grid = new SheetGrid(sheet);

            try {
                Header header = Header.detect(grid);
                ParseContext context = buildGroups(header, grid, faculty);
                parseLessons(header, grid, context);
            } catch (ParserException ex) {
                ExamHeader examHeader = ExamHeader.detect(grid);
                if (examHeader == null) {
                    throw ex;
                }
                ParseContext context = buildGroups(examHeader, grid, faculty);
                parseExamLessons(examHeader, grid, context);
            }
        } finally {
            try {
                wb.close();
            } catch (Exception ignore) {
                // close quietly
            }
        }
    }

    private ParseContext buildGroups(Header header, SheetGrid grid, String faculty) throws ParserException {
        return buildGroups(header.subgroupRow, header.groupRow, header.nameRow, header.subgroupColumns, grid, faculty);
    }

    private ParseContext buildGroups(ExamHeader header, SheetGrid grid, String faculty) throws ParserException {
        return buildGroups(header.subgroupRow, header.groupRow, header.nameRow, header.subgroupColumns, grid, faculty);
    }

    private ParseContext buildGroups(int subgroupRow,
                                     int groupRow,
                                     int nameRow,
                                     List<Integer> subgroupColumns,
                                     SheetGrid grid,
                                     String faculty) throws ParserException {
        Map<Integer, Subgroup> subgroupByCol = new HashMap<>();
        Map<String, Subgroup> subgroupById = new HashMap<>();
        Map<String, Group> groupById = new HashMap<>();
        Map<Integer, String> groupIdByCol = new HashMap<>();

        for (int col : subgroupColumns) {
            String subgroupId = grid.getValue(subgroupRow, col);
            if (subgroupId.isBlank()) {
                continue;
            }
            String groupId = grid.findNearestLeftValue(groupRow, col, subgroupColumns);
            if (groupId.isBlank()) {
                throw new ParserException("table format exception: group id missing for subgroup " + subgroupId);
            }
            String groupName = grid.findNearestLeftValue(nameRow, col, subgroupColumns);
            if (groupName.isBlank()) {
                groupName = groupId;
            }

            Group group = groupById.get(groupId);
            if (group == null) {
                group = new Group();
                group.setId(groupId);
                group.setName(groupName);
                groupById.put(groupId, group);
            }

            Subgroup subgroup = new Subgroup();
            subgroup.setId(subgroupId);
            group.addSubgroup(subgroup);

            subgroupByCol.put(col, subgroup);
            subgroupById.put(subgroupId, subgroup);
            groupIdByCol.put(col, groupId);
        }

        groups.addAll(groupById.values());
        return new ParseContext(subgroupByCol, subgroupById, groupById, groupIdByCol);
    }

    private void parseLessons(Header header, SheetGrid grid, ParseContext context) throws ParserException {
        List<Integer> dayRows = header.dayRows;
        if (dayRows.isEmpty()) {
            throw new ParserException("table format exception: day rows not found");
        }

        for (int i = 0; i < dayRows.size(); i++) {
            int startRow = dayRows.get(i);
            int endRow = (i + 1 < dayRows.size()) ? dayRows.get(i + 1) - 1 : grid.maxRow();

            String day = grid.getValue(startRow, header.dayColumn);
            String date = grid.getValue(startRow, header.dateColumn);

            for (int row = startRow; row <= endRow; row++) {
                String lessonNumber = grid.getValue(row, header.lessonNumberColumn);
                if (!isLessonNumber(lessonNumber)) {
                    continue;
                }

                String timeText = grid.getValue(row + 1, header.lessonNumberColumn);
                TimeRange timeRange = parseTimeRange(timeText);
                if (timeRange == null) {
                    continue;
                }

                Set<String> processedMerged = new HashSet<>();
                for (int col : header.subgroupColumns) {
                    CellRangeAddress merged = grid.getMergedRegion(row, col);
                    if (merged != null) {
                        String key = merged.formatAsString();
                        if (processedMerged.contains(key)) {
                            continue;
                        }
                        processedMerged.add(key);
                    }

                    List<Integer> coveredCols = merged == null
                            ? List.of(col)
                            : header.subgroupColumns.stream()
                            .filter(c -> c >= merged.getFirstColumn() + 1 && c <= merged.getLastColumn() + 1)
                            .toList();

                    if (coveredCols.isEmpty()) {
                        continue;
                    }

                    int sourceCol = coveredCols.get(0);
                    String lessonNameCell = grid.getValue(row, sourceCol);
                    if (lessonNameCell.isBlank()) {
                        continue;
                    }

                    String teacherCell = grid.getValue(row + 1, sourceCol);
                    String auditorium = grid.getValue(row + 2, sourceCol);

                    Map<String, List<String>> subgroupsByGroup = new HashMap<>();
                    for (int coveredCol : coveredCols) {
                        String groupId = context.groupIdByCol.get(coveredCol);
                        if (groupId == null) {
                            continue;
                        }
                        Subgroup subgroup = context.subgroupByCol.get(coveredCol);
                        if (subgroup != null) {
                            subgroupsByGroup.computeIfAbsent(groupId, k -> new ArrayList<>()).add(subgroup.getId());
                        }
                    }

                    for (Map.Entry<String, List<String>> entry : subgroupsByGroup.entrySet()) {
                        String groupId = entry.getKey();
                        Group group = context.groupById.get(groupId);
                        if (group == null) {
                            throw new ParserException("table format exception: cannot resolve group for column " + col);
                        }

                        List<String> subgroupIds = entry.getValue();
                        boolean isCommon = subgroupIds.size() >= 2;

                        Lesson lesson = parseLesson(lessonNameCell);
                        lesson.setDate(date)
                                .setWeekDay(day)
                                .setStartTime(timeRange.start)
                                .setEndTime(timeRange.end)
                                .setId(UUID.randomUUID())
                                .setGroupId(groupId);

                        if (!auditorium.isBlank()) {
                            lesson.setAuditorium(auditorium);
                        }

                        if (isCommon) {
                            group.addLesson(lesson);
                        } else if (subgroupIds.size() == 1) {
                            Subgroup subgroup = context.subgroupById.get(subgroupIds.get(0));
                            if (subgroup != null) {
                                lesson.setSubgroupId(subgroup.getId());
                                subgroup.addLesson(lesson);
                            }
                        }

                        lessons.add(lesson);
                        parseTeachers(teacherCell, lesson);
                    }
                }
            }
        }
    }

    private void parseExamLessons(ExamHeader header, SheetGrid grid, ParseContext context) throws ParserException {
        for (int row = header.dateRow + 1; row <= grid.maxRow(); row++) {
            String dateCell = grid.getValue(row, header.dateColumn);
            if (dateCell.isBlank()) {
                continue;
            }

            DateDay dateDay = parseDateDay(dateCell);
            if (dateDay.date == null && dateDay.day == null) {
                continue;
            }

            Set<String> processedMerged = new HashSet<>();
            for (int col : header.subgroupColumns) {
                CellRangeAddress merged = grid.getMergedRegion(row, col);
                if (merged != null) {
                    String key = merged.formatAsString();
                    if (processedMerged.contains(key)) {
                        continue;
                    }
                    processedMerged.add(key);
                }

                List<Integer> coveredCols = merged == null
                        ? List.of(col)
                        : header.subgroupColumns.stream()
                        .filter(c -> c >= merged.getFirstColumn() + 1 && c <= merged.getLastColumn() + 1)
                        .toList();

                if (coveredCols.isEmpty()) {
                    continue;
                }

                int sourceCol = coveredCols.get(0);
                String cell = grid.getValue(row, sourceCol);
                if (cell.isBlank()) {
                    continue;
                }

                ExamCell examCell = parseExamCell(cell);
                if (examCell.name == null || examCell.name.isBlank()) {
                    continue;
                }

                Map<String, List<String>> subgroupsByGroup = new HashMap<>();
                for (int coveredCol : coveredCols) {
                    String groupId = context.groupIdByCol.get(coveredCol);
                    if (groupId == null) {
                        continue;
                    }
                    Subgroup subgroup = context.subgroupByCol.get(coveredCol);
                    if (subgroup != null) {
                        subgroupsByGroup.computeIfAbsent(groupId, k -> new ArrayList<>()).add(subgroup.getId());
                    }
                }

                for (Map.Entry<String, List<String>> entry : subgroupsByGroup.entrySet()) {
                    String groupId = entry.getKey();
                    Group group = context.groupById.get(groupId);
                    if (group == null) {
                        throw new ParserException("table format exception: cannot resolve group for column " + col);
                    }

                    List<String> subgroupIds = entry.getValue();
                    boolean isCommon = subgroupIds.size() >= 2;

                    Lesson lesson = parseLesson(examCell.name);
                    lesson.setDate(dateDay.date != null ? dateDay.date : "")
                            .setWeekDay(dateDay.day != null ? dateDay.day : "")
                            .setStartTime(examCell.time != null ? examCell.time : "")
                            .setEndTime("")
                            .setId(UUID.randomUUID())
                            .setGroupId(groupId);

                    if (examCell.auditorium != null && !examCell.auditorium.isBlank()) {
                        lesson.setAuditorium(examCell.auditorium);
                    }

                    if (isCommon) {
                        group.addLesson(lesson);
                    } else if (subgroupIds.size() == 1) {
                        Subgroup subgroup = context.subgroupById.get(subgroupIds.get(0));
                        if (subgroup != null) {
                            lesson.setSubgroupId(subgroup.getId());
                            subgroup.addLesson(lesson);
                        }
                    }

                    lessons.add(lesson);
                    if (examCell.teacher != null && !examCell.teacher.isBlank()) {
                        parseTeachers(examCell.teacher, lesson);
                    }
                }
            }
        }
    }

    private void parseTeachers(String raw, Lesson lesson) {
        if (raw == null || raw.isBlank()) {
            return;
        }
        String normalized = normalizeTeacherLine(raw);
        if (normalized.isBlank()) {
            return;
        }
        if (!normalized.contains(",")) {
            Teacher teacher = parseTeacher(normalized);
            if (teacher != null) {
                teacher.addLesson(lesson);
                teachers.add(teacher);
            }
            return;
        }
        for (String token : splitManyTeachersToList(normalized)) {
            Teacher teacher = parseTeacher(token);
            if (teacher != null) {
                teacher.addLesson(lesson);
                teachers.add(teacher);
            }
        }
    }

    private Teacher parseTeacher(String s) {
        String value = s.trim();
        if (value.isBlank()) {
            return null;
        }
        String qualification = "";
        Matcher matcher = QUALIFICATION_PATTERN.matcher(value);
        if (matcher.find()) {
            qualification = matcher.group(1).trim();
            value = value.substring(0, matcher.start()).trim();
        }
        String[] parts = value.split("\\s+");
        String lastName = parts.length > 0 ? parts[0] : value;
        String initials = parts.length > 1
                ? String.join(" ", Arrays.copyOfRange(parts, 1, parts.length))
                : "";
        return new Teacher()
                .setLastname(lastName)
                .setQualification(qualification)
                .setInitials(initials);
    }

    private Lesson parseLesson(String cellValue) {
        String lessonName = parseLessonName(cellValue);
        String lessonType = parseLessonType(cellValue);
        return new Lesson()
                .setName(lessonName)
                .setType(lessonType);
    }

    private List<String> splitManyTeachersToList(String value) {
        return Arrays.stream(value.split(","))
                .map(String::trim)
                .filter(token -> !token.isBlank())
                .toList();
    }

    private String normalizeTeacherLine(String value) {
        String v = value.trim();
        v = v.replaceAll("^(доц\\.|проф\\.|ст\\.преп\\.|ст\\.пр\\.|преп\\.|асс\\.|канд\\.[^\\s]*\\s+|д-р\\s+|докт\\.[^\\s]*\\s+)+", "");
        return v.trim();
    }

    private String parseLessonName(String cellValue) {
        String stripped = cellValue.replaceAll("\\s*\\([^)]*\\)", "").trim();
        return stripped.replaceAll("\\s{2,}", " ");
    }

    private String parseLessonType(String cellValue) {
        Matcher matcher = QUALIFICATION_PATTERN.matcher(cellValue);
        if (!matcher.find()) {
            return "";
        }
        String tag = matcher.group(1).toLowerCase(Locale.ROOT);
        if (tag.contains("лек")) {
            return "лек";
        }
        if (tag.contains("лаб")) {
            return "лаб";
        }
        if (tag.contains("пз")) {
            return "пз";
        }
        if (tag.contains("конс")) {
            return "конс";
        }
        if (tag.contains("экз")) {
            return "экз";
        }
        if (tag.contains("курс")) {
            return "курс";
        }
        return tag;
    }

    public Workbook readWorkbook(File file) throws ParserException {
        try {
            return WorkbookFactory.create(file);
        } catch (Exception e) {
            throw new ParserException("cannot read workbook");
        }
    }

    private boolean isLessonNumber(String value) {
        if (value == null || value.isBlank()) {
            return false;
        }
        for (int i = 0; i < value.length(); i++) {
            if (!Character.isDigit(value.charAt(i))) {
                return false;
            }
        }
        return true;
    }

    private TimeRange parseTimeRange(String value) {
        if (value == null || value.isBlank()) {
            return null;
        }
        Matcher matcher = TIME_RANGE_PATTERN.matcher(value);
        if (!matcher.find()) {
            return null;
        }
        String start = matcher.group(1).replace('.', ':');
        String end = matcher.group(2).replace('.', ':');
        return new TimeRange(start, end);
    }

    private record TimeRange(String start, String end) {}

    private record ParseContext(
            Map<Integer, Subgroup> subgroupByCol,
            Map<String, Subgroup> subgroupById,
            Map<String, Group> groupById,
            Map<Integer, String> groupIdByCol
    ) {}

    private static final class Header {
        private final int subgroupRow;
        private final int groupRow;
        private final int nameRow;
        private final List<Integer> subgroupColumns;
        private final List<Integer> dayRows;
        private final int dayColumn;
        private final int dateColumn;
        private final int lessonNumberColumn;

        private Header(int subgroupRow,
                       int groupRow,
                       int nameRow,
                       List<Integer> subgroupColumns,
                       List<Integer> dayRows,
                       int dayColumn,
                       int dateColumn,
                       int lessonNumberColumn) {
            this.subgroupRow = subgroupRow;
            this.groupRow = groupRow;
            this.nameRow = nameRow;
            this.subgroupColumns = subgroupColumns;
            this.dayRows = dayRows;
            this.dayColumn = dayColumn;
            this.dateColumn = dateColumn;
            this.lessonNumberColumn = lessonNumberColumn;
        }

        private static Header detect(SheetGrid grid) throws ParserException {
            int maxRow = Math.min(grid.maxRow(), 60);
            int maxCol = Math.min(grid.maxCol(), 80);

            List<Integer> bestColumns = List.of();
            int subgroupRow = -1;

            for (int row = 1; row <= maxRow; row++) {
                List<Integer> cols = new ArrayList<>();
                for (int col = 1; col <= maxCol; col++) {
                    String value = grid.getValue(row, col);
                    if (SUBGROUP_PATTERN.matcher(value).matches()) {
                        cols.add(col);
                    }
                }
                if (cols.size() >= 2 && cols.size() > bestColumns.size()) {
                    bestColumns = cols;
                    subgroupRow = row;
                }
            }

            if (subgroupRow < 0) {
                throw new ParserException("table format exception: subgroup row not found");
            }

            int groupRow = findHeaderRowAbove(grid, subgroupRow - 1, bestColumns);
            int nameRow = findHeaderRowAbove(grid, groupRow - 1, bestColumns);

            int dayColumn = detectDayColumn(grid);
            List<Integer> dayRows = findDayRows(grid, dayColumn);
            if (dayRows.isEmpty()) {
                throw new ParserException("table format exception: day rows not found");
            }
            int dateColumn = detectDateColumn(grid, dayRows.get(0), dayColumn);
            int lessonNumberColumn = detectLessonNumberColumn(grid, dayRows.get(0), dateColumn);

            return new Header(
                    subgroupRow,
                    groupRow,
                    nameRow,
                    bestColumns,
                    dayRows,
                    dayColumn,
                    dateColumn,
                    lessonNumberColumn
            );
        }

        private static int findHeaderRowAbove(SheetGrid grid, int startRow, List<Integer> subgroupColumns) throws ParserException {
            for (int row = startRow; row >= 1; row--) {
                boolean any = false;
                for (int col : subgroupColumns) {
                    if (!grid.getValue(row, col).isBlank()) {
                        any = true;
                        break;
                    }
                }
                if (any) {
                    return row;
                }
            }
            throw new ParserException("table format exception: header rows not found");
        }

        private static int detectDayColumn(SheetGrid grid) throws ParserException {
            Map<Integer, Integer> columnHits = new HashMap<>();
            int maxRow = Math.min(grid.maxRow(), 120);
            int maxCol = Math.min(grid.maxCol(), 80);
            for (int row = 1; row <= maxRow; row++) {
                for (int col = 1; col <= maxCol; col++) {
                    String value = grid.getValue(row, col);
                    if (DAY_NAMES.contains(value)) {
                        columnHits.merge(col, 1, Integer::sum);
                    }
                }
            }
            int maxHits = columnHits.values().stream().mapToInt(Integer::intValue).max().orElse(0);
            return columnHits.entrySet()
                    .stream()
                    .filter(entry -> entry.getValue() == maxHits)
                    .map(Map.Entry::getKey)
                    .min(Integer::compareTo)
                    .orElseThrow(() -> new ParserException("table format exception: day column not found"));
        }

        private static List<Integer> findDayRows(SheetGrid grid, int dayColumn) {
            List<Integer> rows = new ArrayList<>();
            for (int row = 1; row <= grid.maxRow(); row++) {
                String value = grid.getValue(row, dayColumn);
                if (DAY_NAMES.contains(value)) {
                    String prev = grid.getValue(row - 1, dayColumn);
                    if (DAY_NAMES.contains(prev) && prev.equals(value)) {
                        continue;
                    }
                    rows.add(row);
                }
            }
            return rows;
        }

        private static int detectDateColumn(SheetGrid grid, int dayRow, int dayColumn) throws ParserException {
            int maxCol = Math.min(grid.maxCol(), 80);
            for (int col = dayColumn + 1; col <= maxCol; col++) {
                String value = grid.getValue(dayRow, col);
                if (looksLikeDate(value)) {
                    return col;
                }
                if (!value.isBlank()) {
                    return col;
                }
            }
            throw new ParserException("table format exception: date column not found");
        }

        private static int detectLessonNumberColumn(SheetGrid grid, int dayRow, int dateColumn) throws ParserException {
            int maxCol = Math.min(grid.maxCol(), 80);
            for (int col = dateColumn + 1; col <= maxCol; col++) {
                String value = grid.getValue(dayRow, col);
                if (isLessonNumber(value)) {
                    return col;
                }
            }
            throw new ParserException("table format exception: lesson number column not found");
        }

        private static boolean isLessonNumber(String value) {
            if (value == null || value.isBlank()) {
                return false;
            }
            for (int i = 0; i < value.length(); i++) {
                if (!Character.isDigit(value.charAt(i))) {
                    return false;
                }
            }
            return true;
        }

        private static boolean looksLikeDate(String value) {
            if (value == null || value.isBlank()) {
                return false;
            }
            String v = value.toLowerCase(Locale.ROOT);
            return v.contains("г.") || v.contains("г ") || v.matches(".*\\d{4}.*");
        }
    }

    private record ExamHeader(
            int subgroupRow,
            int groupRow,
            int nameRow,
            List<Integer> subgroupColumns,
            int dateRow,
            int dateColumn
    ) {
        private static ExamHeader detect(SheetGrid grid) {
            int maxRow = Math.min(grid.maxRow(), 80);
            int maxCol = Math.min(grid.maxCol(), 80);

            int dateRow = -1;
            int dateColumn = -1;
            for (int row = 1; row <= maxRow; row++) {
                for (int col = 1; col <= maxCol; col++) {
                    String value = grid.getValue(row, col).trim();
                    if (value.equalsIgnoreCase("дата")) {
                        dateRow = row;
                        dateColumn = col;
                        break;
                    }
                }
                if (dateRow > 0) {
                    break;
                }
            }

            if (dateRow < 0) {
                return null;
            }

            List<Integer> bestColumns = List.of();
            int subgroupRow = -1;
            for (int row = 1; row <= maxRow; row++) {
                List<Integer> cols = new ArrayList<>();
                for (int col = 1; col <= maxCol; col++) {
                    String value = grid.getValue(row, col);
                    if (SUBGROUP_PATTERN.matcher(value).matches()) {
                        cols.add(col);
                    }
                }
                if (cols.size() >= 2 && cols.size() > bestColumns.size()) {
                    bestColumns = cols;
                    subgroupRow = row;
                }
            }

            if (subgroupRow < 0) {
                return null;
            }

            int groupRow = findHeaderRowAbove(grid, subgroupRow - 1, bestColumns);
            int nameRow = findHeaderRowAbove(grid, groupRow - 1, bestColumns);

            return new ExamHeader(
                    subgroupRow,
                    groupRow,
                    nameRow,
                    bestColumns,
                    dateRow,
                    dateColumn
            );
        }

        private static int findHeaderRowAbove(SheetGrid grid, int startRow, List<Integer> subgroupColumns) {
            for (int row = startRow; row >= 1; row--) {
                boolean any = false;
                for (int col : subgroupColumns) {
                    if (!grid.getValue(row, col).isBlank()) {
                        any = true;
                        break;
                    }
                }
                if (any) {
                    return row;
                }
            }
            return -1;
        }
    }

    private record ExamCell(String name, String teacher, String auditorium, String time) {}

    private ExamCell parseExamCell(String value) {
        String[] rawLines = value.split("\\r?\\n");
        List<String> lines = new ArrayList<>();
        for (String line : rawLines) {
            String trimmed = line.trim();
            if (!trimmed.isBlank()) {
                lines.add(trimmed);
            }
        }
        if (lines.isEmpty()) {
            return new ExamCell("", "", "", "");
        }

        String name = lines.get(0);
        String teacher = "";
        String auditorium = "";
        String time = "";

        for (String line : lines) {
            if (teacher.isBlank() && (line.contains("доц.") || line.contains("проф.") || line.contains("преп.")
                    || TEACHER_LINE_PATTERN.matcher(line).find())) {
                teacher = line;
            }
            if (auditorium.isBlank() && line.toLowerCase(Locale.ROOT).contains("ауд")) {
                int comma = line.indexOf(',');
                auditorium = comma > 0 ? line.substring(0, comma).trim() : line.trim();
                Matcher tm = TIME_ONLY_PATTERN.matcher(line);
                if (tm.find()) {
                    time = tm.group(1).replace('.', ':');
                }
            }
            if (time.isBlank()) {
                Matcher tm = TIME_ONLY_PATTERN.matcher(line);
                if (tm.find()) {
                    time = tm.group(1).replace('.', ':');
                }
            }
        }

        return new ExamCell(name, teacher, auditorium, time);
    }

    private DateDay parseDateDay(String value) {
        String date = null;
        String day = null;
        Matcher dateMatcher = DATE_PATTERN.matcher(value);
        if (dateMatcher.find()) {
            date = dateMatcher.group(1);
        }
        for (String dayName : DAY_NAMES) {
            if (value.contains(dayName)) {
                day = dayName;
                break;
            }
        }
        return new DateDay(date, day);
    }

    private record DateDay(String date, String day) {}

    private static final class SheetGrid {
        private final Sheet sheet;
        private final DataFormatter formatter = new DataFormatter();
        private final List<CellRangeAddress> mergedRegions;

        private SheetGrid(Sheet sheet) {
            this.sheet = sheet;
            this.mergedRegions = collectMergedRegions(sheet);
        }

        private int maxRow() {
            return sheet.getLastRowNum() + 1;
        }

        private int maxCol() {
            int max = 0;
            for (Row row : sheet) {
                if (row.getLastCellNum() > max) {
                    max = row.getLastCellNum();
                }
            }
            return max;
        }

        private String getValue(int rowIndex, int colIndex) {
            if (rowIndex <= 0 || colIndex <= 0) {
                return "";
            }
            Row row = sheet.getRow(rowIndex - 1);
            if (row == null) {
                return "";
            }
            Cell cell = row.getCell(colIndex - 1);
            String raw = cell == null ? "" : formatter.formatCellValue(cell).trim();
            if (!raw.isBlank()) {
                return raw;
            }
            CellRangeAddress merged = getMergedRegion(rowIndex, colIndex);
            if (merged == null) {
                return "";
            }
            Row topRow = sheet.getRow(merged.getFirstRow());
            if (topRow == null) {
                return "";
            }
            Cell topCell = topRow.getCell(merged.getFirstColumn());
            return topCell == null ? "" : formatter.formatCellValue(topCell).trim();
        }

        private CellRangeAddress getMergedRegion(int rowIndex, int colIndex) {
            int r = rowIndex - 1;
            int c = colIndex - 1;
            for (CellRangeAddress region : mergedRegions) {
                if (region.isInRange(r, c)) {
                    return region;
                }
            }
            return null;
        }

        private String findNearestLeftValue(int rowIndex, int colIndex, List<Integer> allowedColumns) {
            int current = colIndex;
            Set<Integer> allowed = new HashSet<>(allowedColumns);
            while (current >= 1) {
                if (!allowed.isEmpty() && !allowed.contains(current)) {
                    current--;
                    continue;
                }
                String value = getValue(rowIndex, current);
                if (!value.isBlank()) {
                    return value;
                }
                current--;
            }
            return "";
        }

        private static List<CellRangeAddress> collectMergedRegions(Sheet sheet) {
            List<CellRangeAddress> regions = new ArrayList<>();
            int count = sheet.getNumMergedRegions();
            for (int i = 0; i < count; i++) {
                regions.add(sheet.getMergedRegion(i));
            }
            return regions;
        }
    }
}
