package com.vsuscheduleweb.mapper;

import com.vsuscheduleweb.DTO.LessonResponse;
import com.vsuscheduleweb.entity.Lesson;
import lombok.RequiredArgsConstructor;
import org.modelmapper.ModelMapper;
import org.springframework.stereotype.Component;

@Component
@RequiredArgsConstructor
public class LessonMapper {
    private final ModelMapper mapper;

    public LessonResponse entityToResponse(Lesson lesson) {
        return mapper.map(lesson, LessonResponse.class);
    }
}
