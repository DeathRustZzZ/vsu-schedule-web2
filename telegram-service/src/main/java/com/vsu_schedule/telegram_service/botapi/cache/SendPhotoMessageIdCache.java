package com.vsu_schedule.telegram_service.botapi.cache;

import lombok.Getter;
import lombok.Setter;
import org.springframework.stereotype.Component;

@Getter
@Setter
@Component
public class SendPhotoMessageIdCache {
    private Integer lastMessageId;
}
