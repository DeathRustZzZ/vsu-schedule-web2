package com.vsu_schedule.telegram_service.botapi.cache;

import com.vsu_schedule.telegram_service.dto.TeacherResponse;
import lombok.Getter;
import org.springframework.stereotype.Component;

import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

@Component
public class TeacherSessionStore {
    @Getter
    private final Map<Long, List<TeacherResponse>> storage = new ConcurrentHashMap<>();

    public void put(Long telegramId, List<TeacherResponse> teacherResponseList) {
        storage.put(telegramId,teacherResponseList);
    }

    public void remove(Long telegramId) {
        storage.remove(telegramId);
    }

    public List<TeacherResponse> get(Long telegramId) {
        return storage.get(telegramId);
    }
}
