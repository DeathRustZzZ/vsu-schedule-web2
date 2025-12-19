package com.vsu_schedule.telegram_service.botapi.config;


import com.vsu_schedule.telegram_service.botapi.ScheduleBot;
import com.vsu_schedule.telegram_service.botapi.TelegramFacade;
import lombok.RequiredArgsConstructor;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.telegram.telegrambots.meta.api.methods.updates.SetWebhook;

@Configuration
@RequiredArgsConstructor
public class ApplicationConfig {

    private final ScheduleBotConfig scheduleBotConfig;

    @Bean
    public SetWebhook setWebhookInstance() {
        return SetWebhook.builder().url(scheduleBotConfig.getBotPath()).build();
    }

    @Bean
    public ScheduleBot springWebHookBot(SetWebhook webhook, TelegramFacade facade) {
        return new ScheduleBot(
                webhook,
                facade,
                scheduleBotConfig.getBotToken(),
                scheduleBotConfig.getBotName(),
                scheduleBotConfig.getBotPath()
        );
    }
}