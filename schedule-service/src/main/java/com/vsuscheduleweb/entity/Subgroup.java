package com.vsuscheduleweb.entity;

import jakarta.persistence.*;
import lombok.*;
import lombok.experimental.Accessors;

import java.util.ArrayList;
import java.util.List;



@Data
@Accessors(chain = true)
@ToString
@Entity(name = "subgroups")
public class Subgroup {

    @Id
    @Column(name = "subgroups_id")
    private String id;

    @OneToMany(cascade = CascadeType.ALL)
    @JoinColumn(name = "subgroup_id", updatable = false)
    private List<Lesson> lessons = new ArrayList<>();

    @Column(name = "group_id")
    private String groupId;
    public void addLesson(Lesson lesson){
        lessons.add(lesson);
    }
}
