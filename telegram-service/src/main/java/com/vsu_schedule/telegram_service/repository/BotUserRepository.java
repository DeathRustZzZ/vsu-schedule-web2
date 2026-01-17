package com.vsu_schedule.telegram_service.repository;

import com.vsu_schedule.telegram_service.entity.BotUser;
import org.springframework.data.jpa.repository.JpaRepository;

public interface BotUserRepository extends JpaRepository<BotUser, Long> {
    BotUser findByTelegramId(Long telegramId);
    Boolean existsByTelegramId(Long telegramId);
}
