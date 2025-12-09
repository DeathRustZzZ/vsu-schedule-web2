package com.vsuscheduleweb.parser;

import com.vsuscheduleweb.Exceptions.ParserException;
import lombok.NoArgsConstructor;
import org.apache.poi.ss.usermodel.Cell;
import org.apache.poi.ss.usermodel.Sheet;
import org.apache.poi.ss.usermodel.Workbook;
import org.apache.poi.ss.usermodel.WorkbookFactory;
import com.vsuscheduleweb.entity.*;
import org.springframework.stereotype.Component;

import java.io.File;
import java.util.*;
import java.util.stream.Collectors;


@NoArgsConstructor
@Component
public  class Parser {

    private List<Teacher> teachers = new ArrayList<>();
    private List<Group> groups = new ArrayList<>();

    private List<Lesson> lessons = new ArrayList<>();
    private ParserStage parsMode;

    public List<Teacher> getTeachers() {
        for(int i = 0; i < teachers.size(); i++){
            teachers.get(i).getLessons().removeIf(lesson -> lesson.getName().equals(""));
        }
        return teachers;
    }

    public List<Group> getGroups() {
        for(int i = 0; i < groups.size(); i++){
            groups.get(i).getCommonLessons().removeIf(lesson -> lesson.getName().equals(""));
            for(int j = 0; j < groups.get(i).getSubgroups().size(); j++){
                groups.get(i).getSubgroups().get(j).getLessons().removeIf(lesson -> lesson.getName().equals(""));
            }
        }
        return groups;
    }

    public List<Lesson> getLessons(){
        return lessons;
    }

    private  List<Cell> returnWorkspace(Workbook wb){
        List<Cell> cells = getAllCellsWhereBorderLeftExist(wb);

        cells.removeIf(cell -> (cell.getStringCellValue().contains("курс") || cell.getStringCellValue().contains("№")) ||
                (((cell.getColumnIndex() == 2 || cell.getColumnIndex() == 0 || cell.getColumnIndex() == 1)
                        && cell.getStringCellValue().equals(""))) ||
                cell.getRowIndex() < 12);


        return cells;
    }

    private  List<Cell> getAllCellsWhereBorderLeftExist(Workbook wb){
        Sheet st = wb.getSheetAt(0);
        List<Cell> cells = new ArrayList<>();
        st.forEach(row->{
            row.forEach(
                    cell -> {
                        try {
                            cell.getStringCellValue();
                            if (cell.getCellStyle().getBorderLeft() > 0)
                                cells.add(cell);
                            if(cell.getColumnIndex() == 1)
                                cells.add(cell);
                        }catch (IllegalStateException e){
                            cell.setCellValue(String.valueOf((int)cell.getNumericCellValue()));
                            cells.add(cell);
                        }

                    }
            );

        });
        return cells;
    }
    private void print(List<Cell> cells){
        for(int l = 0; l < cells.size(); l++){
            System.out.println(cells.get(l).getStringCellValue() + " | " + cells.get(l).getColumnIndex() + " | " + cells.get(l).getCellStyle().getBorderLeft());
        }
    }

    private String[] splitTimeCellToArr(Cell cell){
        List<String> arr = Arrays.stream(cell.getStringCellValue().split(" "))
                .collect(Collectors.toList());
        arr.removeIf(x -> x.equals(""));
        return arr.get(arr.size() - 1).split("-");
    }

    private String parseStartTime(Cell cell){
        return splitTimeCellToArr(cell)[0].replace("(","")
                .replace(".",":");
    }

    private String parseEndTime(Cell cell){
        String[] arr = splitTimeCellToArr(cell);
        return arr[arr.length - 1].replace(")","")
                .replace(".",":");
    }

    private  HashMap<Integer,Subgroup> parseGroups(List<Cell> workspace, String facult) throws ParserException {
        HashMap<Integer,Subgroup> map = new HashMap<>();
        Queue<Group> queue = new ArrayDeque<>();
        Queue<Group> queueCache = new ArrayDeque<>();
        int count = 0;
        for(int i = 0; true; i++) {

            if (parsMode == ParserStage.PARSER_STAGE_NAME_OF_GROUPS) {
                if (workspace.get(i + 1).getStringCellValue().equals("")) {
                    throw new ParserException("table format Exception.");
                }
                Group group = new Group().setName(workspace.get(i).getStringCellValue());
                if(!(workspace.get(i+1).getColumnIndex() == 3 && workspace.get(i + 1).getCellStyle().getBorderLeft() == 5))
                    group.setCountOfSubGroups(workspace.get(i+1).getColumnIndex() - workspace.get(i).getColumnIndex());
                else
                    group.setCountOfSubGroups(workspace.get(i).getColumnIndex() - workspace.get(i-1).getColumnIndex());
                groups.add(group);
                queue.add(group);
                count++;
                if(workspace.get(i+1).getColumnIndex() == 3 && workspace.get(i + 1).getCellStyle().getBorderLeft() == 5){
                    parsMode = ParserStage.PARSER_STAGE_GROUPS_ID;
                }
                continue;
            }
            if (parsMode == ParserStage.PARSER_STAGE_GROUPS_ID) {
                Group group = queue.poll();
                if(group != null) {
                    queueCache.add(group);
                    group.setId(workspace.get(i).getStringCellValue() + "/" + facult);
                }
                if(workspace.get(i+1).getColumnIndex() == 3 && workspace.get(i + 1).getCellStyle().getBorderLeft() > 3){
                    for(int j = 0; j < count; j++)
                        queue.add(queueCache.poll());
                    parsMode = ParserStage.PARSER_STAGE_SUBGROUPS;
                }
                continue;
            }
            if (parsMode == ParserStage.PARSER_STAGE_SUBGROUPS) {
                Group group = queue.poll();
                int lastIndex = 0;
                if(group != null) {
                    for (int index = 0; index < group.getCountOfSubGroups(); index++) {
                        Subgroup subgroup = new Subgroup().setId(workspace.get(i + index).getStringCellValue());
                        map.put(workspace.get(i + index).getColumnIndex(), subgroup);
                        group.addSubgroup(subgroup);
                        lastIndex = index;
                    }
                }
                if(workspace.get(i+1).getColumnIndex() == 0 && workspace.get(i + 1).getCellStyle().getBorderLeft() > 3){
                    parsMode = ParserStage.PARSER_STAGE_SKIP;
                    return map;
                }
                i += lastIndex;

            }
        }

    }

    public void parse(File xlsFile , String facult) throws ParserException{
        groups = new ArrayList<>();
        teachers = new ArrayList<>();
        Workbook wb = readWorkbook(xlsFile);
        List<Cell> workspace = returnWorkspace(wb);
        final int length = getLengthOfWorkspace(workspace);
        parsMode = ParserStage.PARSER_STAGE_NAME_OF_GROUPS;
        HashMap<Integer,Subgroup> map = parseGroups(workspace, facult);
        String day = "";
        String date = "";
        String startTime = "";
        String endTime = "";
        Queue<Lesson> queueOfLessons = new ArrayDeque<>();
        Queue<Lesson> queueCache = new ArrayDeque<>();
        int countOfLessons = 0;
        int skip = 0;
        for(int i = 0; i < workspace.size() - 1 ; i++){

            if (parsMode == ParserStage.PARSER_STAGE_SKIP) {
                if(workspace.get(i + 1).getCellStyle().getBorderLeft() > 3) {
                    skip++;
                    if(skip == 3) {
                        parsMode = ParserStage.PARSER_STAGE_DAY;
                        continue;
                    }
                } else
                    continue;

            }
            if(workspace.get(i).getColumnIndex() == 2 && workspace.get(i).getCellStyle().getBorderLeft() >3){
                parsMode = ParserStage.PARSER_STAGE_TIME;
            }

            if(parsMode == ParserStage.PARSER_STAGE_DAY){
                day = workspace.get(i).getStringCellValue();
                parsMode = ParserStage.PARSER_STAGE_DATE;
                continue;
            }
            if(parsMode == ParserStage.PARSER_STAGE_TIME){
                startTime = parseStartTime(workspace.get(i));
                endTime = parseEndTime(workspace.get(i));
                parsMode = ParserStage.PARSER_STAGE_LESSONS;
                continue;
            }
            if(parsMode == ParserStage.PARSER_STAGE_DATE){
                date = workspace.get(i).getStringCellValue();
                parsMode = ParserStage.PARSER_STAGE_TIME;
                continue;
            }
            if(parsMode == ParserStage.PARSER_STAGE_LESSONS ) {
                Lesson lesson = parseLesson(workspace.get(i));
                lesson .setDate(date)
                        .setWeekDay(day)
                        .setStartTime(startTime)
                        .setEndTime(endTime)
                        .setId(UUID.randomUUID());
                lessons.add(lesson);
                queueOfLessons.add(lesson);
                countOfLessons++;
                if ((isAdjacentColumns(workspace.get(i),workspace.get(i + 1)) ||
                        isPenultimateColumnAndTheNextIsTheFirst(workspace.get(i),workspace.get(i+1),length)) ||
                        isSingleColumnInTheRow(workspace.get(i),workspace.get(i+1))) {

                    int firstColumnIndex = workspace.get(i).getColumnIndex();
                    int secondColumnIndex = workspace.get(i + 1).getColumnIndex();
                    if(firstColumnIndex == 3 && secondColumnIndex == 3){
                        secondColumnIndex = length + 3;
                    }
                    for(int j = firstColumnIndex; j < secondColumnIndex; j++) {
                        Optional<Group> opt_group = findGroupBySubgroupIndex(map.get(j).getId());
                        if (opt_group.isPresent()) {
                            Group group = opt_group.get();
                            if (isThisLessonInGroup(lesson, group)) {
                                continue;
                            }
                            group.addLesson(lesson);
                        } else
                            throw new ParserException("table format exception");
                    }

                } else {
                    try {
                        map.get(workspace.get(i).getColumnIndex()).addLesson(lesson);
                    } catch(NullPointerException e){
                            System.out.println(workspace.get(i).getStringCellValue());
                        }
                }
                if(workspace.get(i + 1).getColumnIndex() == 3){
                    parsMode = ParserStage.PARSER_STAGE_TEACHERS;
                    continue;
                }
            }if(parsMode == ParserStage.PARSER_STAGE_TEACHERS){
                Lesson lesson = queueOfLessons.poll();
                if(lesson != null) {
                    queueCache.add(lesson);
                    if (!workspace.get(i).getStringCellValue().equals("")) {
                        if (!workspace.get(i).getStringCellValue().contains(",")) {
                            Teacher teacher = parseTeacher(workspace.get(i).getStringCellValue());
                            teachers.add(teacher);
                            teacher.addLesson(lesson);
                        } else {
                            for (String teacherString : splitManyTeachersToList(workspace.get(i))) {
                                Teacher teacher = parseTeacher(teacherString);

                                teacher.addLesson(lesson);
                            }
                        }
                    }

                }
                if(workspace.get(i + 1).getColumnIndex() == 3){
                    parsMode = ParserStage.PARSER_STAGE_AUDITORIUMS;
                    for(int j = 0; j < countOfLessons  ; j++){
                        if(queueCache.peek() != null)
                            queueOfLessons.add(queueCache.poll());
                        else
                            break;
                    }
                    continue;
                }
            }if(parsMode == ParserStage.PARSER_STAGE_AUDITORIUMS){
                Lesson lesson = queueOfLessons.poll();
                if(lesson == null) {
                    if(workspace.get(i+1).getCellStyle().getBorderLeft() == 5 && workspace.get(i + 1).getColumnIndex() == 0 ) {
                        parsMode = ParserStage.PARSER_STAGE_DAY;
                    }
                    countOfLessons = 0;
                    continue;
                };
                lesson.setAuditorium(workspace.get(i).getStringCellValue());
            }

        }
    }

    private Teacher parseTeacher(String s){
        String teacherLastName = parseTeacherLastname(s);
        String teacherInitials = parseTeacherInitials(s);
        String teacherQualification = parseTeacherQualification(s);
        return new Teacher()
                        .setLastname(teacherLastName)
                        .setQualification(teacherQualification)
                        .setInitials(teacherInitials);


    }


    private Lesson parseLesson(Cell cell){
        String lessonName = parseLessonName(cell);
        String lessonType = parseLessonType(cell);
        return new Lesson()
                .setName(lessonName)
                .setType(lessonType);
    }
    private List<String> splitManyTeachersToList(Cell cell){
        return Arrays.stream(cell.getStringCellValue().split(",")).toList();
    }

    private String[] splitTeacherStringToArr(String s){
        return s.split(" ");

    }

    private String parseTeacherLastname(String s){
        String[] arr = splitTeacherStringToArr(s);
        return arr[0];
    }

    private String parseTeacherQualification(String s){
        String[] arr = splitTeacherStringToArr(s);
        return arr[arr.length - 1].replace("(","").replace(")","");
    }

    private String parseTeacherInitials(String s){
        String[] arr = splitTeacherStringToArr(s);
        return arr[1];
    }



    private boolean isSingleColumnInTheRow(Cell cell, Cell nextCell){
        return (nextCell.getColumnIndex() == 3  && cell.getColumnIndex() == 3);
    }

    private boolean isAdjacentColumns(Cell cell, Cell nextCell){
        return (nextCell.getColumnIndex() - cell.getColumnIndex()) >= 2;

    }
    private boolean isPenultimateColumnAndTheNextIsTheFirst(Cell cell, Cell nextCell, int lengthOfWorkSpace){
        return ((cell.getColumnIndex() == lengthOfWorkSpace  + 1) && nextCell.getColumnIndex() == 3);
    }
    private String parseLessonName(Cell cell){
        List<String> lessonNameArr = Arrays.stream(cell.getStringCellValue().split(" "))
                .filter(x -> !(x.contains("(") && x.contains(")"))).collect(Collectors.toList());
        String lessonName = "";
        for(String word : lessonNameArr ){
            if(!word.equals(""))
                lessonName += word + " ";
        }
        return lessonName;
    }

    private String parseLessonType(Cell cell){
        String cellValue = cell.getStringCellValue();
        String type = "";
        if(cellValue.contains("(лк)"))
            type = "лк";
        if(cellValue.contains("(пз)"))
            type = "пз";
        if(cellValue.contains("(лаб)"))
            type = "лаб";
        return type;
    }

    private boolean isThisLessonInGroup(Lesson lesson, Group group){
        for(Lesson groupLesson : group.getCommonLessons()){
            if(groupLesson.getStartTime().equals(lesson.getStartTime()) && groupLesson.getDate().equals(lesson.getDate())){
                return true;
            }
        }
        return false;

    }

    public Workbook readWorkbook(File file) throws ParserException{
        try {
            Workbook wb = WorkbookFactory.create(file);
            return wb;
        }
        catch (Exception e) {
            throw new ParserException("cannot read workbook");


        }
    }


    private Optional<Group> findGroupBySubgroupIndex(String index){
        for(int i = 0; i < groups.size(); i++ ){
            for(int j = 0; j < groups.get(i).getSubgroups().size(); j++ ){
                if(groups.get(i).getSubgroups().get(j).getId().equals(index)){
                    return Optional.of(groups.get(i));
                }
            }

        }
        return Optional.empty();
    }

    private int getLengthOfWorkspace(List<Cell> cells){ // only after returnWorkspace()
        int length = 0;
        for(int i = 1; true; i++) {
            if (cells.get(i).getCellStyle().getBorderLeft() >= 3)
                break;
            length++;
        }
        return (length +1)* 2;
    }


}
