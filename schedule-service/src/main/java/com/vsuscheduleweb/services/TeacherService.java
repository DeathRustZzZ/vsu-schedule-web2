package com.vsuscheduleweb.services;

import com.vsuscheduleweb.DTO.ListTeacherResponse;
import com.vsuscheduleweb.DTO.TeacherResponse;
import com.vsuscheduleweb.Exceptions.TeacherNotFoundException;
import com.vsuscheduleweb.entity.Teacher;
import com.vsuscheduleweb.mapper.TeacherMapper;
import com.vsuscheduleweb.repositories.TeacherRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.http.ResponseEntity;
import org.springframework.stereotype.Service;

import java.util.List;
import java.util.UUID;

@Service
@RequiredArgsConstructor
public class TeacherService {
    private final TeacherRepository teacherRepository;
    private final TeacherMapper mapper;

    public TeacherResponse getById(Integer id) {
        return mapper.entityToResponse(teacherRepository.findById(id).orElseThrow(
                () -> new TeacherNotFoundException(String.format("teacher with id: %s is not found.", id))
        ));
    }

    public ListTeacherResponse getAll() {
        return new ListTeacherResponse(
                teacherRepository.findAll().stream().map(mapper::entityToResponse).toList()
        );
    }
}
