package com.vsu_schedule.telegram_service.botapi.service;


import com.vsu_schedule.telegram_service.botapi.TelegramActionEvent;
import com.vsu_schedule.telegram_service.botapi.callback_query_types.ResetRegistrationCallbackQueryTypes;
import com.vsu_schedule.telegram_service.dto.GroupWithSubgroupsIds;
import com.vsu_schedule.telegram_service.dto.LessonResponse;
import com.vsu_schedule.telegram_service.dto.ListGroupWithSubgroupsIds;
import com.vsu_schedule.telegram_service.dto.ListLessonResponse;
import com.vsu_schedule.telegram_service.entity.BotUser;
import com.vsu_schedule.telegram_service.feign.GroupFeignClient;
import com.vsu_schedule.telegram_service.feign.LessonFeignClient;
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

    private final LessonFeignClient lessonFeignClient;

    public BotApiMethod<?> handleFacultyCallbackQuery(CallbackQuery callbackQuery) {
        removeInlineMarkup(callbackQuery);
        BotUser user = botUserRepository.findByTelegramId(callbackQuery.getFrom().getId()).get();
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

    public BotApiMethod<?> handleGroupCallbackQuery(CallbackQuery callbackQuery) {
        removeInlineMarkup(callbackQuery);
        BotUser user = botUserRepository.findByTelegramId(callbackQuery.getFrom().getId()).get();
        String[] callbackQuerySplitArr = callbackQuery.getData().split("\\.");
        user.setGroupId(callbackQuerySplitArr[1].replaceAll("/","."));
        user.setSubgroupId(callbackQuerySplitArr[callbackQuerySplitArr.length - 1]);
        botUserRepository.save(user);
        return new SendMessage(callbackQuery.getMessage().getChatId().toString(),
                "Вы были успешно зарегистрированы!");

    }

    public BotApiMethod<?> handleWeekDayCallbackQuery(CallbackQuery query) {
        removeInlineMarkup(query);
        String weekDay =  query.getData().split("\\.")[1];
        BotUser botUser = botUserRepository.findByTelegramId(query.getFrom().getId()).get();
        ListLessonResponse lessonResponse = lessonFeignClient.getLessonsByGroupAndSubgroupAndWeekDay(
                botUser.getGroupId(),
                botUser.getSubgroupId(),
                weekDay);
        return SendMessage.builder()
                .text(buildLessonsStringWithWeekDay(lessonResponse,weekDay))
                .chatId(botUser.getChatId()).build();


    }

    private String buildLessonsStringWithWeekDay(ListLessonResponse response, String weekDay) {
        List<LessonResponse> lessons = response.getLessonResponses();

        if (lessons.isEmpty()) {
            return "📅 *Расписание на " + weekDay + "*\n\nНикаких занятий не найдено 😴";
        }

        String date = lessons.get(0).getDate();
        StringBuilder sb = new StringBuilder();

        sb.append("📅 *Расписание на ").append(weekDay).append("*")
                .append(" (").append(date).append(")\n\n");

        for (int i = 0; i < lessons.size(); i++) {
            LessonResponse lesson = lessons.get(i);

            sb.append(String.format("%d. *%s* (%s)\n", i + 1, lesson.getName(), lesson.getType()))
                    .append(String.format("⏰ `%s — %s`\n", lesson.getStartTime(), lesson.getEndTime()))
                    .append(String.format("📍 _%s_\n\n", lesson.getAuditorium()));
        }

        return sb.toString();
    }

    private InlineKeyboardMarkup getGroupInlineKeyboard(String faculty) {
        ListGroupWithSubgroupsIds listGroupWithSubgroupsIds = groupFeignClient.getAvailableGroupsByFaculty(faculty);
        List<InlineKeyboardRow> inlineKeyboardRows = new ArrayList<>();
        for(GroupWithSubgroupsIds group : listGroupWithSubgroupsIds.getListGroupWithSubgroupsIds()) {
            for(String subgroupId : group.getSubgroupIds()) {
                inlineKeyboardRows.add(new InlineKeyboardRow(
                        InlineKeyboardButton.builder()
                        .callbackData("group." + group.getGroupId().replaceAll("\\." , "/") +"." +  subgroupId)
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