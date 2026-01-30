package com.vsu_schedule.telegram_service.botapi.cache;


import lombok.Getter;
import org.springframework.stereotype.Component;

import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

@Getter
@Component
public class LastMessageIdCache {

    private final Map<Long, Integer> storage = new ConcurrentHashMap<>();

    public void put(Long chatId, Integer messageId) {
        storage.put(chatId,messageId);
    }

    public void remove(Long chatId) {
        storage.remove(chatId);
    }

    public Integer get(Long chatId) {
        return storage.get(chatId);
    }
}
