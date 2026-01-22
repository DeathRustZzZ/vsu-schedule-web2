package com.vsuscheduleweb.Controllers;

import com.vsuscheduleweb.DTO.ListLessonResponse;
import com.vsuscheduleweb.services.LessonService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/api/v1/lessons")
@RequiredArgsConstructor
public class LessonController {

    public final LessonService lessonService;

    @GetMapping
    public ListLessonResponse getAll() {
        return lessonService.getAll();
    }

    @GetMapping("/{groupId}/{subgroupId}")
    public ListLessonResponse getLessonsByGroupAndSubgroup(@PathVariable String groupId, @PathVariable String subgroupId) {
        return lessonService.getLessonsByGroupAndSubgroup(groupId,subgroupId);
    }

    @GetMapping("/{groupId}/{subgroupId}/{weekDay}")
    public ListLessonResponse getLessonsByGroupAndSubgroupAndWeekDay(@PathVariable String groupId,
                                                                     @PathVariable String subgroupId,
                                                                     @PathVariable String weekDay){
        return lessonService.getLessonsByGroupAndSubgroupAndWeekDay(groupId,subgroupId,weekDay);
    }
}
