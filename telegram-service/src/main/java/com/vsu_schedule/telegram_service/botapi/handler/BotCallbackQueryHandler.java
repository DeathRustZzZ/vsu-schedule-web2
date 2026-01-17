package com.vsu_schedule.telegram_service.botapi.handler;

import com.vsu_schedule.telegram_service.botapi.callback_query_types.FacultyCallbackQueryTypes;
import com.vsu_schedule.telegram_service.botapi.service.BotCallbackQueryService;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.apache.logging.log4j.message.Message;
import org.springframework.stereotype.Component;
import org.telegram.telegrambots.meta.api.methods.BotApiMethod;
import org.telegram.telegrambots.meta.api.objects.CallbackQuery;

@Component
@RequiredArgsConstructor
@Slf4j
public class BotCallbackQueryHandler {

    private final BotCallbackQueryService botCallbackQueryService;

    public BotApiMethod<?> handle(CallbackQuery query) {
        log.info("callbackQueryHandler.handle method invoke.");
        if(query.getData().contains("faculty")) {
            return botCallbackQueryService.handleFacultyCallbackQuery(query);
        }
        return null;
    }

}
