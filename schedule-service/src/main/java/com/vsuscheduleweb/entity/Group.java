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
@Entity(name = "groups")
public class Group {
    @Id
    @Column(name = "group_id")
    private String id;

    @OneToMany(cascade = CascadeType.ALL, orphanRemoval = true)
    @JoinColumn(name = "group_id", updatable = false)
    private List<Lesson> commonLessons = new ArrayList<>();

    @OneToMany(cascade = CascadeType.ALL)
    @JoinColumn(name = "group_id", updatable = false)
    private List<Subgroup> subgroups = new ArrayList<>();
    @Column(name = "group_name")
    private String name;

    private transient int countOfSubGroups;

    public void addLesson(Lesson lesson){
        commonLessons.add(lesson);
    }

    public void addSubgroup(Subgroup subgroup){
        subgroups.add(subgroup);
    }

}
