package com.vsuscheduleweb.mapper;

import com.vsuscheduleweb.DTO.SubGroupResponse;
import com.vsuscheduleweb.entity.Subgroup;
import lombok.RequiredArgsConstructor;
import org.modelmapper.ModelMapper;
import org.springframework.stereotype.Component;

@Component
@RequiredArgsConstructor
public class SubgroupMapper {
    private final ModelMapper mapper;
    private final LessonMapper lessonMapper;

    public SubGroupResponse entityToResponse(Subgroup subgroup){
        return mapper.map(subgroup,SubGroupResponse.class)
                .setLessons(subgroup.getLessons().stream().map(lessonMapper::entityToResponse).toList());
    }
}
