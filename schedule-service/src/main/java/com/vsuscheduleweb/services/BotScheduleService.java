package com.vsuscheduleweb.services;

import com.vsuscheduleweb.DTO.ListLessonResponse;
import com.vsuscheduleweb.mapper.LessonMapper;
import com.vsuscheduleweb.repositories.LessonRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;

@Service
@RequiredArgsConstructor
public class BotScheduleService {
    private final BotScheduleCacheService cacheService;
    private final LessonRepository lessonRepository;
    private final LessonMapper lessonMapper;

    public ListLessonResponse getSchedule(String faculty, String groupId, String subgroupId, String weekDay) {
        return cacheService
                .get(faculty, groupId, subgroupId, weekDay)
                .orElseGet(() -> {
                    var lessons = lessonRepository
                            .findByFacultyAndGroupOrSubgroupAndWeekDay(faculty, groupId, subgroupId, weekDay)
                            .stream()
                            .map(lessonMapper::entityToResponse)
                            .toList();
                    ListLessonResponse response = new ListLessonResponse(lessons);
                    cacheService.put(faculty, groupId, subgroupId, weekDay, response);
                    return response;
                });
    }
}
