package com.vsuscheduleweb.entity;


import jakarta.persistence.*;
import lombok.*;
import lombok.experimental.Accessors;

import java.util.ArrayList;
import java.util.List;
import java.util.UUID;


@Data
@Accessors(chain = true)
@ToString
@Entity(name = "lessons")
public class Lesson {

    @Id
    @Column(name = "lesson_id")
    private UUID id;
    @Column(name = "start_time")
    private String startTime;

    @Column(name = "end_time")
    private String endTime;

    private String auditorium;


    private String date;

    @Column(name = "weekday")
    private String weekDay;

    @Column(name = "group_id")
    private String groupId;

    @Column(name = "teacher_id")
    private Integer teacherId;

    @Column(name = "subgroup_id")
    private String subgroupId;

    @Column(name = "lesson_name")
    private String name;

    private String type;

    private String facult;

}
