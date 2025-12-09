package com.vsuscheduleweb.mapper;

import com.vsuscheduleweb.DTO.GroupResponse;
import com.vsuscheduleweb.entity.Group;
import lombok.RequiredArgsConstructor;
import org.modelmapper.ModelMapper;
import org.springframework.stereotype.Component;

@Component
@RequiredArgsConstructor
public class GroupMapper {
    private final ModelMapper modelMapper;
    private final LessonMapper lessonMapper;
    private final SubgroupMapper subgroupMapper;

    public GroupResponse entityToResponse(Group group){
        return modelMapper.map(group, GroupResponse.class)
                .setCommonLessons(
                        group.getCommonLessons().stream().map(lessonMapper::entityToResponse).toList()
                )
                .setSubgroups(
                        group.getSubgroups().stream().map(subgroupMapper::entityToResponse).toList()
                );

    }
}
