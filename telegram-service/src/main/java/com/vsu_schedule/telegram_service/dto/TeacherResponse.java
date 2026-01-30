package com.vsu_schedule.telegram_service.dto;

import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;
import lombok.experimental.Accessors;

@Getter
@Setter
@NoArgsConstructor
@AllArgsConstructor
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
    private String qualification;
}
