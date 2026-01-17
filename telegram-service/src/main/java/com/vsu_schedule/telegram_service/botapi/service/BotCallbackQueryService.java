package com.vsu_schedule.telegram_service.botapi.service;


import com.vsu_schedule.telegram_service.botapi.TelegramActionEvent;
import com.vsu_schedule.telegram_service.botapi.callback_query_types.ResetRegistrationCallbackQueryTypes;
import com.vsu_schedule.telegram_service.entity.BotUser;
import com.vsu_schedule.telegram_service.repository.BotUserRepository;
import jakarta.transaction.Transactional;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.context.ApplicationEventPublisher;
import org.springframework.stereotype.Component;
import org.telegram.telegrambots.meta.api.methods.AnswerCallbackQuery;
import org.telegram.telegrambots.meta.api.methods.BotApiMethod;
import org.telegram.telegrambots.meta.api.methods.send.SendMessage;
import org.telegram.telegrambots.meta.api.methods.updatingmessages.EditMessageReplyMarkup;
import org.telegram.telegrambots.meta.api.objects.CallbackQuery;
import org.telegram.telegrambots.meta.api.objects.replykeyboard.InlineKeyboardMarkup;

@Component
@RequiredArgsConstructor
@Slf4j
public class BotCallbackQueryService {

    private final BotUserRepository botUserRepository;

    private final ApplicationEventPublisher applicationEventPublisher;

    public BotApiMethod<?> handleFacultyCallbackQuery(CallbackQuery callbackQuery) {
        BotUser user = botUserRepository.findByTelegramId(callbackQuery.getFrom().getId());
        user.setFaculty(callbackQuery.getData().split("\\.")[1]);
        SendMessage sendMessage = new SendMessage();
        sendMessage.setText("Выберете группу");
        sendMessage.setChatId(user.getChatId());
        removeInlineMarkup(callbackQuery);
        return sendMessage;
    }

    @Transactional
    public BotApiMethod<?> handleResetRegistrationQuery(CallbackQuery callbackQuery) {
            removeInlineMarkup(callbackQuery);
            if(callbackQuery.getData().equals(ResetRegistrationCallbackQueryTypes.YES.toString())) {
                botUserRepository.deleteByTelegramId(callbackQuery.getFrom().getId());
                SendMessage sendMessage = new SendMessage();
                sendMessage.setText("Ваш аккаунт был удален для прохождения регистрации отправьте команду /register.");
                sendMessage.setChatId(callbackQuery.getMessage().getChatId());
                return sendMessage;
            }
            return null;
    }



    private InlineKeyboardMarkup getGroupInlineKeyboard() {
        return null; // TODO добавить автоматический запрос списка групп по факультету на микросервис schedule-service вида List<Groups> ScheduleService.getGroups(String faculty)
                    // TODO можно кешировать ответ запроса на определенное время что бы снять нагрузкку с schedule-service
    }


    private void removeInlineMarkup(CallbackQuery callbackQuery) {
        answerCallBackQuery(callbackQuery);
        EditMessageReplyMarkup editMessageReplyMarkup = EditMessageReplyMarkup.builder()
                .chatId(callbackQuery.getMessage().getChatId())
                .messageId(callbackQuery.getMessage().getMessageId())
                .replyMarkup(null)
                .build();
        applicationEventPublisher.publishEvent(new TelegramActionEvent(this, editMessageReplyMarkup));
    }

    private void answerCallBackQuery(CallbackQuery callbackQuery) {
        AnswerCallbackQuery answerCallbackQuery = AnswerCallbackQuery.builder()
                .callbackQueryId(callbackQuery.getId())
                .build();
        applicationEventPublisher.publishEvent(new TelegramActionEvent(this, answerCallbackQuery));
    }
}