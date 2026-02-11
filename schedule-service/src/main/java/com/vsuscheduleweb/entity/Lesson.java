package com.vsuscheduleweb.entity;


import jakarta.persistence.*;
import lombok.*;
import lombok.experimental.Accessors;
import lombok.extern.slf4j.Slf4j;

import java.util.ArrayList;
import java.util.List;
import java.util.UUID;


@Data
@Accessors(chain = true)
@ToString
@Entity(name = "lessons")
@Slf4j
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

    @ManyToOne(fetch = FetchType.LAZY)
    @JoinColumn(name = "teacher_id", insertable = false, updatable = false)
    @ToString.Exclude
    @EqualsAndHashCode.Exclude
    private Teacher teacher;

    @Column(name = "subgroup_id")
    private String subgroupId;

    @Column(name = "lesson_name")
    private String name;

    private String type;

    private String faculty;

    // Переопределяем сеттер для валидации длины аудитории
    public Lesson setAuditorium(String auditorium) {
        if (auditorium != null && auditorium.length() > 30) {
            log.warn("Auditorium too long ({}), truncating: '{}'", auditorium.length(), auditorium);
            this.auditorium = auditorium.substring(0, 30);
        } else {
            this.auditorium = auditorium;
        }
        return this;
    }

}
