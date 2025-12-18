package com.vsuscheduleweb.Controllers;


import com.vsuscheduleweb.DTO.ListTeacherResponse;
import com.vsuscheduleweb.DTO.TeacherResponse;
import com.vsuscheduleweb.services.TeacherService;
import lombok.RequiredArgsConstructor;
import org.springframework.http.HttpStatus;
import org.springframework.web.bind.annotation.*;

import java.util.UUID;


@RequiredArgsConstructor
@RequestMapping("/rest/teachers")
@RestController()
public class TeacherController {

    private final TeacherService teacherService;

    @GetMapping
    @ResponseStatus(HttpStatus.OK)
    public ListTeacherResponse getTeachers() {
        return teacherService.getAll();
    }

    @GetMapping("/{id}")
    @ResponseStatus(HttpStatus.OK)
    public TeacherResponse getById(@PathVariable UUID id) {
        return teacherService.getById(id);
    }

}
