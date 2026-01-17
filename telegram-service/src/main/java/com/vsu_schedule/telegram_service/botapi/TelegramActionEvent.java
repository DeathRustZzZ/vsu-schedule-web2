package com.vsu_schedule.telegram_service.botapi;

import lombok.Getter;
import org.springframework.context.ApplicationEvent;
import org.telegram.telegrambots.meta.api.methods.BotApiMethod;

@Getter
public class TelegramActionEvent extends ApplicationEvent {
    private final BotApiMethod<?> method;

    public TelegramActionEvent(Object source, BotApiMethod<?> method) {
        super(source);
        this.method = method;
    }
}
