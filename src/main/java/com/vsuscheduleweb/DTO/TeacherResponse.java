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
public class TeacherResponse {
    private Integer id;
    private String firstname;
    private String lastname;
    private String surname;
    private String initials;
    private String imgLink;
    private String description;
    private String fullname;
    private List<LessonResponse> lessons = new ArrayList<>();
    private String qualification;
}
