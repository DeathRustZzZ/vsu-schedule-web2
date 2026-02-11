package com.vsuscheduleweb.mapper;

import com.vsuscheduleweb.DTO.LessonResponse;
import com.vsuscheduleweb.entity.Lesson;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Component;

@Component
@RequiredArgsConstructor
public class LessonMapper {
    private final TeacherMapper teacherMapper;

    public LessonResponse entityToResponse(Lesson lesson) {
        LessonResponse response = LessonResponse.builder()
                .id(lesson.getId())
                .startTime(lesson.getStartTime())
                .endTime(lesson.getEndTime())
                .auditorium(lesson.getAuditorium())
                .date(lesson.getDate())
                .weekDay(lesson.getWeekDay())
                .groupId(lesson.getGroupId())
                .teacherId(lesson.getTeacherId())
                .subgroupId(lesson.getSubgroupId())
                .name(lesson.getName())
                .type(lesson.getType())
                .build();
        if (lesson.getTeacher() != null) {
            response.setTeacher(teacherMapper.entityToResponse(lesson.getTeacher()));
        }
        return response;
    }
}
