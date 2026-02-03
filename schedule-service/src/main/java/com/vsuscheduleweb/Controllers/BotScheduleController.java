package com.vsuscheduleweb.Controllers;

import com.vsuscheduleweb.DTO.ListLessonResponse;
import com.vsuscheduleweb.services.BotScheduleService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/api/v1/bot")
@RequiredArgsConstructor
public class BotScheduleController {
    private final BotScheduleService botScheduleService;

    @GetMapping("/schedule")
    public ListLessonResponse getSchedule(
            @RequestParam String faculty,
            @RequestParam String group,
            @RequestParam String subgroup,
            @RequestParam String weekday
    ) {
        return botScheduleService.getSchedule(faculty, group, subgroup, weekday);
    }
}
