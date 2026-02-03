package com.vsuscheduleweb.entity;

import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.Id;
import jakarta.persistence.Table;
import lombok.Data;
import lombok.experimental.Accessors;

import java.time.LocalDate;
import java.util.UUID;

@Data
@Accessors(chain = true)
@Entity
@Table(name = "app_user")
public class AppUser {

    @Id
    @Column(name = "user_id")
    private UUID id;

    @Column(name = "created_at")
    private LocalDate createdAt;

    @Column(name = "login")
    private String login;

    @Column(name = "lastname")
    private String lastName;

    @Column(name = "name")
    private String firstName;

    @Column(name = "email")
    private String email;

    @Column(name = "password")
    private String passwordHash;

    @Column(name = "status")
    private String status;
}
