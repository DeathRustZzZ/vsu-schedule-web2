package com.vsu_schedule.telegram_service.botapi;

import com.vsu_schedule.telegram_service.botapi.handler.BotCallbackQueryHandler;
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
    private final BotCallbackQueryHandler callbackQueryHandler;

    public BotApiMethod<?> handleUpdate(Update req) {
        if(req.hasMessage()){

            Message message = req.getMessage();
            if(message.getText().toCharArray()[0] == '/') {
                return commandHandler.handle(message);
            }
        }
        if(req.hasCallbackQuery()){
            log.info("req.hasCallbackQuery");
            return callbackQueryHandler.handle(req.getCallbackQuery());
        }
        return null;
    }
}
