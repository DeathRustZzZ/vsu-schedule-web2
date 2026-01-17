package com.vsu_schedule.telegram_service.botapi.service;


import com.vsu_schedule.telegram_service.entity.BotUser;
import com.vsu_schedule.telegram_service.repository.BotUserRepository;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Component;
import org.telegram.telegrambots.meta.api.methods.BotApiMethod;
import org.telegram.telegrambots.meta.api.methods.send.SendMessage;
import org.telegram.telegrambots.meta.api.objects.CallbackQuery;
import org.telegram.telegrambots.meta.api.objects.replykeyboard.InlineKeyboardMarkup;

@Component
@RequiredArgsConstructor
@Slf4j
public class BotCallbackQueryService {

    private final BotUserRepository botUserRepository;

    public BotApiMethod<?> handleFacultyCallbackQuery(CallbackQuery callbackQuery) {
        BotUser user = botUserRepository.findByTelegramId(callbackQuery.getFrom().getId());
        user.setFaculty(callbackQuery.getData().split("\\.")[1]);
        SendMessage sendMessage = new SendMessage();
        sendMessage.setText("Выберете группу");
        sendMessage.setChatId(user.getChatId());
        // sendMessage.setReplyMarkup(getGroupInlineKeyboard());
        return sendMessage;
    }


    private InlineKeyboardMarkup getGroupInlineKeyboard() {
        return null; // TODO добавить автоматический запрос списка групп по факультету на микросервис schedule-service вида List<Groups> ScheduleService.getGroups(String faculty)
                    // TODO можно кешировать ответ запроса на определенное время что бы снять нагрузкку с schedule-service
    }
}