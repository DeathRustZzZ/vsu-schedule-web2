package com.vsuscheduleweb.DTO;

import lombok.*;

import java.util.UUID;


@Getter
@Setter
@NoArgsConstructor
@AllArgsConstructor
@Builder
public class LessonResponse {
    private UUID id;
    private String startTime;
    private String endTime;
    private String auditorium;
    private String date;
    private String weekDay;
    private String groupId;
    private Integer teacherId;
    private String subgroupId;
    private String name;
    private String type;
    private String facult;
}
