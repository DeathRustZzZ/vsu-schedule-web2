package com.vsuscheduleweb.DTO;


import com.vsuscheduleweb.entity.Lesson;
import lombok.*;
import lombok.experimental.Accessors;

import java.util.ArrayList;
import java.util.List;


@Getter
@Setter
@NoArgsConstructor
@AllArgsConstructor
@Builder
@Accessors(chain = true)
public class SubGroupResponse {
    private String id;
    private List<LessonResponse> lessons = new ArrayList<>();
    private String groupId;
}
