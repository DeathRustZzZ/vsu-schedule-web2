package com.vsu_schedule.telegram_service.entity;

import com.vsu_schedule.telegram_service.entity.enums.BotState;
import jakarta.persistence.*;
import lombok.Getter;
import lombok.Setter;
import lombok.experimental.Accessors;

import java.util.UUID;


@Getter
@Setter
@Entity
@Table(name = "bot-users")
@Accessors(chain = true)
public class BotUser {
    @Id
    @GeneratedValue(strategy = GenerationType.UUID)
    @Column(name = "user-id", nullable = false)
    private UUID id;
    @Column(name = "telegram-user-id", nullable = false)
    private Long telegramId;
    @Column(name = "chat-id", nullable = false)
    private Long chatId;
    @Column(name = "username")
    private String username;
    @Column(name = "faculty")
    private String faculty;
    @Column(name = "group-id")
    private String groupId;
    @Column(name = "subgroup-id")
    private String subgroupId;
    @Enumerated(EnumType.STRING)
    @Column(name = "bot-state")
    private BotState botState = BotState.DEFAULT;
}