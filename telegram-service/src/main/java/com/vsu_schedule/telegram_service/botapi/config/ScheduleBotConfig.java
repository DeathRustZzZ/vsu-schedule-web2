package com.vsu_schedule.telegram_service.botapi.config;

import lombok.Getter;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.PropertySource;
import org.springframework.stereotype.Controller;

@Controller
@PropertySource("application.yaml")
@Getter
public class ScheduleBotConfig {
    @Value("${bot.token}")
    private String botToken;
    @Value("${bot.name}")
    private String botName;
    @Value("${bot.path}")
    private String botPath;
}
