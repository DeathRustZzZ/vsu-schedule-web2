package com.vsu_schedule.telegram_service.botapi.config;

import lombok.Getter;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.PropertySource;
import org.springframework.stereotype.Component;

@Component
@PropertySource("application.yaml")
@Getter
public class BotConfig {
    @Value("${bot.token}")
    private String botToken;
}
