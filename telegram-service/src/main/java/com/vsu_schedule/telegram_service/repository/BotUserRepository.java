package com.vsu_schedule.telegram_service.repository;

import com.vsu_schedule.telegram_service.entity.BotUser;
import org.springframework.data.jpa.repository.JpaRepository;

import java.util.Optional;

public interface BotUserRepository extends JpaRepository<BotUser, Long> {
    Optional<BotUser> findByTelegramId(Long telegramId);
    Boolean existsByTelegramId(Long telegramId);
    void deleteByTelegramId(Long telegramId);
}
