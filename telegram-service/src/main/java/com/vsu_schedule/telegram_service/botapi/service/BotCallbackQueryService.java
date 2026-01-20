package com.vsu_schedule.telegram_service.botapi.service;


import com.vsu_schedule.telegram_service.botapi.TelegramActionEvent;
import com.vsu_schedule.telegram_service.botapi.callback_query_types.ResetRegistrationCallbackQueryTypes;
import com.vsu_schedule.telegram_service.dto.GroupWithSubgroupsIds;
import com.vsu_schedule.telegram_service.dto.ListGroupWithSubgroupsIds;
import com.vsu_schedule.telegram_service.entity.BotUser;
import com.vsu_schedule.telegram_service.feign.GroupFeignClient;
import com.vsu_schedule.telegram_service.repository.BotUserRepository;
import jakarta.transaction.Transactional;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.context.ApplicationEventPublisher;
import org.springframework.stereotype.Component;
import org.telegram.telegrambots.meta.api.methods.AnswerCallbackQuery;
import org.telegram.telegrambots.meta.api.methods.botapimethods.BotApiMethod;
import org.telegram.telegrambots.meta.api.methods.send.SendMessage;
import org.telegram.telegrambots.meta.api.methods.updatingmessages.EditMessageReplyMarkup;
import org.telegram.telegrambots.meta.api.objects.CallbackQuery;
import org.telegram.telegrambots.meta.api.objects.replykeyboard.InlineKeyboardMarkup;
import org.telegram.telegrambots.meta.api.objects.replykeyboard.buttons.InlineKeyboardButton;
import org.telegram.telegrambots.meta.api.objects.replykeyboard.buttons.InlineKeyboardRow;

import java.util.ArrayList;
import java.util.List;

@Component
@RequiredArgsConstructor
@Slf4j
public class BotCallbackQueryService {

    private final BotUserRepository botUserRepository;

    private final ApplicationEventPublisher applicationEventPublisher;

    private final GroupFeignClient groupFeignClient;

    public BotApiMethod<?> handleFacultyCallbackQuery(CallbackQuery callbackQuery) {
        removeInlineMarkup(callbackQuery);
        BotUser user = botUserRepository.findByTelegramId(callbackQuery.getFrom().getId());
        String faculty = callbackQuery.getData().split("\\.")[1];
        user.setFaculty(faculty);
        SendMessage sendMessage = new SendMessage(user.getChatId().toString(),"Выберете группу");
        sendMessage.setReplyMarkup(getGroupInlineKeyboard(faculty));
        return sendMessage;
    }

    @Transactional
    public BotApiMethod<?> handleResetRegistrationQuery(CallbackQuery callbackQuery) {
            removeInlineMarkup(callbackQuery);
            if(callbackQuery.getData().equals(ResetRegistrationCallbackQueryTypes.YES.toString())) {
                botUserRepository.deleteByTelegramId(callbackQuery.getFrom().getId());
                return new SendMessage(callbackQuery.getMessage().getChatId().toString(),
                        "Ваш аккаунт был удален для прохождения регистрации отправьте команду /register.");
            }
            return null;
    }



    private InlineKeyboardMarkup getGroupInlineKeyboard(String faculty) {
        ListGroupWithSubgroupsIds listGroupWithSubgroupsIds = groupFeignClient.getAvailableGroupsByFaculty(faculty);
        List<InlineKeyboardRow> inlineKeyboardRows = new ArrayList<>();
        for(GroupWithSubgroupsIds group : listGroupWithSubgroupsIds.getListGroupWithSubgroupsIds()) {
            for(String subgroupId : group.getSubgroupIds()) {
                inlineKeyboardRows.add(new InlineKeyboardRow(
                        InlineKeyboardButton.builder()
                        .callbackData("group." + group.getGroupId() +"." +  subgroupId)
                        .text(group.getGroupId() + "/" +subgroupId)
                        .build()));
            }
        }

        return InlineKeyboardMarkup.builder()
                .keyboard(inlineKeyboardRows)
                .build();
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