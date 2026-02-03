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
    private static final Pattern TIME_RANGE_PATTERN = Pattern.compile("(\\d{1,2}[:.]\\d{2})\\s*-\\s*(\\d{1,2}[:.]\\d{2})");
    private static final Pattern QUALIFICATION_PATTERN = Pattern.compile("\\(([^)]*)\\)\\s*$");

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
        Sheet sheet = wb.getSheetAt(0);
        SheetGrid grid = new SheetGrid(sheet);

        Header header = Header.detect(grid);
        ParseContext context = buildGroups(header, grid, faculty);
        parseLessons(header, grid, context);
    }

    private ParseContext buildGroups(Header header, SheetGrid grid, String faculty) throws ParserException {
        Map<Integer, Subgroup> subgroupByCol = new HashMap<>();
        Map<String, Group> groupById = new HashMap<>();
        Map<Integer, String> groupIdByCol = new HashMap<>();

        for (int col : header.subgroupColumns) {
            String subgroupId = grid.getValue(header.subgroupRow, col);
            if (subgroupId.isBlank()) {
                continue;
            }
            String groupId = grid.findNearestLeftValue(header.groupRow, col, header.subgroupColumns);
            if (groupId.isBlank()) {
                throw new ParserException("table format exception: group id missing for subgroup " + subgroupId);
            }
            String groupName = grid.findNearestLeftValue(header.nameRow, col, header.subgroupColumns);
            if (groupName.isBlank()) {
                groupName = groupId;
            }

            Group group = groupById.get(groupId);
            if (group == null) {
                group = new Group();
                group.setId(groupId + "/" + faculty);
                group.setName(groupName);
                groupById.put(groupId, group);
            }

            Subgroup subgroup = new Subgroup();
            subgroup.setId(subgroupId);
            group.addSubgroup(subgroup);

            subgroupByCol.put(col, subgroup);
            groupIdByCol.put(col, groupId);
        }

        groups.addAll(groupById.values());
        return new ParseContext(subgroupByCol, groupById, groupIdByCol);
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

                    String lessonNameCell = grid.getValue(row, col);
                    if (lessonNameCell.isBlank()) {
                        continue;
                    }

                    Lesson lesson = parseLesson(lessonNameCell);
                    lesson.setDate(date)
                            .setWeekDay(day)
                            .setStartTime(timeRange.start)
                            .setEndTime(timeRange.end)
                            .setId(UUID.randomUUID());

                    boolean isCommon = false;
                    String groupId = context.groupIdByCol.get(col);
                    Group group = groupId == null ? null : context.groupById.get(groupId);
                    if (merged != null) {
                        List<Integer> coveredCols = header.subgroupColumns.stream()
                                .filter(c -> c >= merged.getFirstColumn() + 1 && c <= merged.getLastColumn() + 1)
                                .toList();
                        if (coveredCols.size() >= 2) {
                            isCommon = true;
                            int leaderCol = coveredCols.get(0);
                            String leaderGroupId = context.groupIdByCol.get(leaderCol);
                            group = leaderGroupId == null ? null : context.groupById.get(leaderGroupId);
                        }
                    }

                    if (group == null) {
                        throw new ParserException("table format exception: cannot resolve group for column " + col);
                    }

                    lesson.setGroupId(group.getId());
                    String auditorium = grid.getValue(row + 2, col);
                    if (!auditorium.isBlank()) {
                        lesson.setAuditorium(auditorium);
                    }

                    if (isCommon) {
                        group.addLesson(lesson);
                    } else {
                        Subgroup subgroup = context.subgroupByCol.get(col);
                        if (subgroup != null) {
                            lesson.setSubgroupId(subgroup.getId());
                            subgroup.addLesson(lesson);
                        }
                    }

                    lessons.add(lesson);
                    parseTeachers(grid.getValue(row + 1, col), lesson);
                }
            }
        }
    }

    private void parseTeachers(String raw, Lesson lesson) {
        if (raw == null || raw.isBlank()) {
            return;
        }
        if (!raw.contains(",")) {
            Teacher teacher = parseTeacher(raw);
            if (teacher != null) {
                teacher.addLesson(lesson);
                teachers.add(teacher);
            }
            return;
        }
        for (String token : splitManyTeachersToList(raw)) {
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

            List<Integer> dayRows = new ArrayList<>();
            for (int row = 1; row <= grid.maxRow(); row++) {
                String value = grid.getValue(row, 4);
                if (DAY_NAMES.contains(value)) {
                    dayRows.add(row);
                }
            }

            return new Header(subgroupRow, groupRow, nameRow, bestColumns, dayRows, 4, 5, 6);
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
    }

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
