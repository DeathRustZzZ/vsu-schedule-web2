package com.vsuscheduleweb.services;

import com.vsuscheduleweb.DTO.ListLessonResponse;
import com.vsuscheduleweb.mapper.LessonMapper;
import com.vsuscheduleweb.repositories.LessonRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;

import java.util.List;


@Service
@RequiredArgsConstructor
public class LessonService {

    public final LessonRepository lessonRepository;

    public final LessonMapper lessonMapper;

    public ListLessonResponse getAll() {
        return new ListLessonResponse(lessonRepository.findAll().stream().map(lessonMapper::entityToResponse).toList());
    }

    public ListLessonResponse getLessonsByGroupAndSubgroup(String groupId, String subgroupId) {
        return new ListLessonResponse(lessonRepository
                .findByGroupIdOrSubgroupId(groupId, subgroupId)
                .stream()
                .map(lessonMapper::entityToResponse).toList());
    }
}
