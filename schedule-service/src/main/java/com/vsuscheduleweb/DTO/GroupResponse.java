package com.vsuscheduleweb.DTO;

import com.vsuscheduleweb.entity.Lesson;
import com.vsuscheduleweb.entity.Subgroup;
import jakarta.persistence.*;
import lombok.*;
import lombok.experimental.Accessors;

import java.util.ArrayList;
import java.util.List;

@Getter
@Setter
@NoArgsConstructor
@AllArgsConstructor
@Accessors(chain = true)
public class GroupResponse {
    private String id;
    private List<LessonResponse> commonLessons = new ArrayList<>();
    private List<SubGroupResponse> subgroups = new ArrayList<>();
    private String name;
    private transient int countOfSubGroups;
}
