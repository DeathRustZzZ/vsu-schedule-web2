package com.vsu_schedule.telegram_service.botapi;

import com.vsu_schedule.telegram_service.botapi.handler.BotMessageCommandHandler;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Component;
import org.telegram.telegrambots.meta.api.methods.BotApiMethod;
import org.telegram.telegrambots.meta.api.objects.Message;
import org.telegram.telegrambots.meta.api.objects.Update;

@Component
@RequiredArgsConstructor
@Slf4j
public class TelegramFacade {

    private final BotMessageCommandHandler commandHandler;

    public BotApiMethod<?> handleUpdate(Update req) {
        if(req.hasMessage()){
            log.info("adddd");
            Message message = req.getMessage();
            Long chatId = message.getChatId();
            if(message.getText().toCharArray()[0] == '/'){
                log.info("asdsadsad");
                return commandHandler.handle(message);
            }
        }
        return null;
    }
}
