package com.vsu_schedule.telegram_service.botapi.event;

import lombok.Getter;
import org.springframework.context.ApplicationEvent;
import org.telegram.telegrambots.meta.api.methods.send.SendPhoto;

@Getter
public class TelegramSendPhotoEvent extends ApplicationEvent {
    private final SendPhoto method;

    public TelegramSendPhotoEvent(Object source, SendPhoto method) {
        super(source);
        this.method = method;
    }
}
