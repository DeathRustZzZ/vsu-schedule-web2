package com.vsu_schedule.telegram_service.botapi.handler;

import com.vsu_schedule.telegram_service.botapi.callback_query_types.ScheduleCallBackQueryTypes;
import com.vsu_schedule.telegram_service.botapi.callback_query_types.TeacherDescriptionCallBackQueryTypes;
import com.vsu_schedule.telegram_service.botapi.callback_query_types.TeacherListCallBackQueryTypes;
import com.vsu_schedule.telegram_service.botapi.service.BotCallbackQueryService;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Component;

import org.telegram.telegrambots.meta.api.methods.botapimethods.BotApiMethod;
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
        if(query.getData().contains("reset.registration.btn")) {
            return botCallbackQueryService.handleResetRegistrationQuery(query);
        }
        if(query.getData().contains("group")) {
            return botCallbackQueryService.handleGroupCallbackQuery(query);
        }
        if(query.getData().contains("weekDay")) {
            return botCallbackQueryService.handleWeekDayCallbackQuery(query);
        }
        if(query.getData().equals(ScheduleCallBackQueryTypes.BACK.getType())) {
            return botCallbackQueryService.handleScheduleBackCallBackQuery(query);
        }
        if(query.getData().equals(ScheduleCallBackQueryTypes.TEACHER_LIST.getType())) {
            return botCallbackQueryService.handleTeacherListCallBackQuery(query);
        }
        if(query.getData().equals(TeacherListCallBackQueryTypes.BACK.getType())) {
            return botCallbackQueryService.handleTeacherListBackCallBackQuery(query);
        }
        if(query.getData().contains("teacher.")){
            return botCallbackQueryService.handleTeacherListSelectCallBackQuery(query);
        }
        return null;
    }

}
