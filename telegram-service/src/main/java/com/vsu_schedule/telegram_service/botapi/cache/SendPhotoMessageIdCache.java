package com.vsu_schedule.telegram_service.botapi.cache;

import lombok.Getter;
import org.springframework.stereotype.Component;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

@Getter
@Component
public class SendPhotoMessageIdCache {
    private final Map<Long, Integer> storage = new ConcurrentHashMap<>();

    public void put(Long telegramId, Integer messageId) {
        storage.put(telegramId,messageId);
    }

    public void remove(Long chatId) {
        storage.remove(chatId);
    }

    public Integer get(Long chatId) {
        return storage.get(chatId);
    }
}
