package com.vsuscheduleweb.mapper;

import com.vsuscheduleweb.DTO.TeacherResponse;
import com.vsuscheduleweb.entity.Teacher;
import lombok.RequiredArgsConstructor;
import org.modelmapper.ModelMapper;
import org.springframework.stereotype.Component;

@Component
@RequiredArgsConstructor
public class TeacherMapper {
    private final ModelMapper mapper;

    public TeacherResponse entityToResponse(Teacher teacher){
        return mapper.map(teacher, TeacherResponse.class);
    }
}
