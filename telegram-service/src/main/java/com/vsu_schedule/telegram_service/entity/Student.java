package com.vsu_schedule.telegram_service.entity;

import jakarta.persistence.*;
import lombok.Getter;
import lombok.Setter;


@Getter
@Setter
@Entity
@Table(name = "students")
public class Student {
    @Id
    @GeneratedValue(strategy = GenerationType.AUTO)
    @Column(name = "student-id", nullable = false)
    private Long id;
    @Column(name = "telegram-user-id", nullable = false)
    private Long telegramId;
    @Column(name = "chat-id", nullable = false)
    private Long chatId;
    @Column(name = "username", nullable = false)
    private String username;
    @Column(name = "faculty", nullable = false)
    private String faculty;
    @Column(name = "group-id", nullable = false)
    private String groupId;
    @Column(name = "subgroup-id", nullable = false)
    private String subgroupId;
}