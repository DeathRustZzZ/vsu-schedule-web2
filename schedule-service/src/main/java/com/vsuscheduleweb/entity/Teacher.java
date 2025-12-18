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
@Entity(name = "teachers")
public class Teacher {

    @Id
    @Column(name = "teacher_id")
    private Integer id;
    private String firstname;
    private String lastname;
    private String surname;
    private String initials;
    @Column(name = "img_link")
    private String imgLink;

    private String description;
    private String fullname;



    @OneToMany(cascade = CascadeType.ALL)
    @JoinColumn(name = "teacher_id", updatable = false)
    private List<Lesson> lessons = new ArrayList<>();
    private String qualification;
    public Teacher addLesson(Lesson lesson){
        lessons.add(lesson);
        return this;
    }

}
