package com.vsuscheduleweb.Controllers;


import com.vsuscheduleweb.DTO.ListTeacherResponse;
import com.vsuscheduleweb.DTO.TeacherResponse;
import com.vsuscheduleweb.services.TeacherParserService;
import com.vsuscheduleweb.services.TeacherService;
import lombok.RequiredArgsConstructor;
import org.springframework.http.HttpStatus;
import org.springframework.web.bind.annotation.*;

import java.util.UUID;


@RequiredArgsConstructor
@RestController()
@RequestMapping("/api/v1/teachers")
public class TeacherController {

    private final TeacherService teacherService;

    private final TeacherParserService teacherParserService;

    @GetMapping
    @ResponseStatus(HttpStatus.OK)
    public ListTeacherResponse getTeachers() {
        return teacherService.getAll();
    }

    @GetMapping("/{id}")
    @ResponseStatus(HttpStatus.OK)
    public TeacherResponse getById(@PathVariable Integer id) {
        return teacherService.getById(id);
    }

    @GetMapping("/parse")
    @ResponseStatus(HttpStatus.OK)
    public ListTeacherResponse parseTeachers() {
        return teacherParserService.parseTeachers();
    }

}
