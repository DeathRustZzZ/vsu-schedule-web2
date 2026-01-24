package com.vsu_schedule.telegram_service.botapi.service;


import com.vsu_schedule.telegram_service.botapi.event.TelegramActionEvent;
import com.vsu_schedule.telegram_service.botapi.event.TelegramSendPhotoEvent;
import com.vsu_schedule.telegram_service.botapi.cache.SendPhotoMessageIdCache;
import com.vsu_schedule.telegram_service.botapi.cache.TeacherSessionStore;
import com.vsu_schedule.telegram_service.botapi.callback_query_types.ResetRegistrationCallbackQueryTypes;
import com.vsu_schedule.telegram_service.botapi.command.Command;
import com.vsu_schedule.telegram_service.dto.*;
import com.vsu_schedule.telegram_service.entity.BotUser;
import com.vsu_schedule.telegram_service.feign.GroupFeignClient;
import com.vsu_schedule.telegram_service.feign.LessonFeignClient;
import com.vsu_schedule.telegram_service.feign.TeacherFeignClient;
import com.vsu_schedule.telegram_service.repository.BotUserRepository;
import jakarta.transaction.Transactional;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.context.ApplicationEventPublisher;
import org.springframework.http.*;
import org.springframework.stereotype.Component;
import org.springframework.web.client.RestTemplate;
import org.telegram.telegrambots.meta.api.methods.AnswerCallbackQuery;
import org.telegram.telegrambots.meta.api.methods.botapimethods.BotApiMethod;
import org.telegram.telegrambots.meta.api.methods.send.SendMessage;
import org.telegram.telegrambots.meta.api.methods.send.SendPhoto;
import org.telegram.telegrambots.meta.api.methods.updatingmessages.DeleteMessage;
import org.telegram.telegrambots.meta.api.methods.updatingmessages.EditMessageReplyMarkup;
import org.telegram.telegrambots.meta.api.methods.updatingmessages.EditMessageText;
import org.telegram.telegrambots.meta.api.objects.CallbackQuery;
import org.telegram.telegrambots.meta.api.objects.InputFile;


import java.io.ByteArrayInputStream;
import java.io.InputStream;
import java.util.*;

import static com.vsu_schedule.telegram_service.botapi.keyboard.MessageKeyboards.*;


@Component
@RequiredArgsConstructor
@Slf4j
public class BotCallbackQueryService {

    private final BotUserRepository botUserRepository;

    private final ApplicationEventPublisher applicationEventPublisher;

    private final GroupFeignClient groupFeignClient;

    private final LessonFeignClient lessonFeignClient;

    private final TeacherFeignClient teacherFeignClient;

    private final SendPhotoMessageIdCache sendPhotoMessageIdCache;

    private final TeacherSessionStore teacherSessionStore;

    public BotApiMethod<?> handleFacultyCallbackQuery(CallbackQuery callbackQuery) {
        removeInlineMarkup(callbackQuery);
        BotUser user = botUserRepository.findByTelegramId(callbackQuery.getFrom().getId()).get();
        ListGroupWithSubgroupsIds listGroupWithSubgroupsIds = groupFeignClient.getAvailableGroupsByFaculty(callbackQuery.getData().split("\\.")[1]);
        String faculty = callbackQuery.getData().split("\\.")[1];
        user.setFaculty(faculty);
        SendMessage sendMessage = new SendMessage(user.getChatId().toString(),"Выберете группу");
        sendMessage.setReplyMarkup(getGroupInlineKeyboard(faculty,listGroupWithSubgroupsIds));
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
        answerCallBackQuery(query);
        String weekDay =  query.getData().split("\\.")[1];
        BotUser botUser = botUserRepository.findByTelegramId(query.getFrom().getId()).get();
        ListLessonResponse lessonResponse = lessonFeignClient.getLessonsByGroupAndSubgroupAndWeekDay(
                botUser.getGroupId(),
                botUser.getSubgroupId(),
                weekDay);
        Map<Integer,TeacherResponse> teacherResponseByLessonId = getTeachersForLessons(lessonResponse);
        teacherSessionStore.put(botUser.getTelegramId(),new ArrayList<>(teacherResponseByLessonId.values()));
        return EditMessageText.builder()
                .messageId(query.getMessage().getMessageId())
                .text(buildScheduleString(lessonResponse,weekDay,teacherResponseByLessonId))
                .chatId(query.getMessage().getChatId())
                .replyMarkup(getScheduleInlineKeyboard())
                .build();

    }

    public BotApiMethod<?> handleTeacherListCallBackQuery(CallbackQuery query) {
        answerCallBackQuery(query);
        return EditMessageReplyMarkup.builder()
                .messageId(query.getMessage().getMessageId())
                .chatId(query.getMessage().getChatId())
                .replyMarkup(getTeacherListInlineKeyboard(teacherSessionStore.get(query.getFrom().getId())))
                .build();
    }

    public BotApiMethod<?> handleTeacherListBackCallBackQuery(CallbackQuery query) {
        answerCallBackQuery(query);
        deleteTeacherDescriptionImage(query.getMessage().getChatId());
        return EditMessageReplyMarkup.builder()
                .messageId(query.getMessage().getMessageId())
                .chatId(query.getMessage().getChatId())
                .replyMarkup(getScheduleInlineKeyboard())
                .build();
    }

    public BotApiMethod<?> handleTeacherListSelectCallBackQuery(CallbackQuery callbackQuery) {
        answerCallBackQuery(callbackQuery);
        List<TeacherResponse> teachersFromCache = teacherSessionStore.get(callbackQuery.getFrom().getId());
        String selectedTeacherId = callbackQuery.getData().split("\\.")[1];
        TeacherResponse selectedTeacher = teachersFromCache.stream()
                .filter(teacher -> teacher.getId().toString().equals(selectedTeacherId))
                .findFirst()
                .orElse(null);
        deleteTeacherDescriptionImage(callbackQuery.getMessage().getChatId());
        if((selectedTeacher != null) && (selectedTeacher.getImgLink() != null)) {
            InputFile image = getImageByLink(selectedTeacher.getImgLink());
            if(image != null){
                SendPhoto sendPhoto = SendPhoto.builder()
                        .chatId(callbackQuery.getMessage().getChatId())
                        .caption(buildTeacherDescriptionMessage(selectedTeacher))
                        .parseMode("HTML")
                        .photo(image)
                        .build();
                applicationEventPublisher.publishEvent(new TelegramSendPhotoEvent(this,sendPhoto));
                return null;
            }
        }
        return null;
    }

    private void deleteTeacherDescriptionImage(Long chatId) {
        if(sendPhotoMessageIdCache.get(chatId) != null) {
            DeleteMessage deleteMessage = DeleteMessage.builder()
                    .messageId(sendPhotoMessageIdCache.get(chatId))
                    .chatId(chatId)
                    .build();
            applicationEventPublisher.publishEvent(new TelegramActionEvent(this,deleteMessage));
        }
    }

    private InputFile getImageByLink(String imgLink) {
        try {
            RestTemplate restTemplate = new RestTemplate();
            HttpHeaders headers = new HttpHeaders();
            headers.setAccept(List.of(MediaType.APPLICATION_OCTET_STREAM, MediaType.IMAGE_JPEG, MediaType.IMAGE_PNG));
            headers.set("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36");
            HttpEntity<String> entity = new HttpEntity<>(headers);
            ResponseEntity<byte[]> response = restTemplate.exchange(
                    imgLink,
                    HttpMethod.GET,
                    entity,
                    byte[].class
            );

            if (response.getStatusCode().is2xxSuccessful() && response.getBody() != null) {
                InputStream is = new ByteArrayInputStream(response.getBody());
                return new InputFile(is, "teacher_photo.jpg");
            }

        } catch (Exception e) {
            log.error("Не удалось скачать фото по ссылке {}: {}", imgLink, e.getMessage());
        }
        return null;
    }



    public BotApiMethod<?> handleScheduleBackCallBackQuery(CallbackQuery query) {
        answerCallBackQuery(query);
        teacherSessionStore.remove(query.getFrom().getId());
        sendPhotoMessageIdCache.remove(query.getMessage().getChatId());
        Optional<BotUser> opt_user = botUserRepository.findByTelegramId(query.getFrom().getId());
        if(opt_user.isPresent()) {
            BotUser user = opt_user.get();
            ListLessonResponse lessonResponse = lessonFeignClient.getLessonsByGroupAndSubgroup(user.getGroupId(), user.getSubgroupId());
            return EditMessageText.builder()
                    .chatId(query.getMessage().getChatId())
                    .messageId(query.getMessage().getMessageId())
                    .replyMarkup(getDayOfWeekSelectInlineKeyboard(lessonResponse))
                    .text("Выберете день недели")
                    .build();
        }else {
            return EditMessageText.builder()
                    .chatId(query.getMessage().getChatId())
                    .messageId(query.getMessage().getMessageId())
                    .replyMarkup(null)
                    .text(Command.getAnswerTextForUnregisteredUsers(query.getFrom().getFirstName()))
                    .build();
        }
    }

    private String buildTeacherDescriptionMessage(TeacherResponse teacher) {
        return String.format("%s\n", teacher.getFullname()) +
                String.format("Квалификация: %s\n", teacher.getQualification()) +
                String.format("Описание: %s\n", teacher.getDescription());

    }

    private String buildScheduleString(ListLessonResponse response, String weekDay,
                                       Map<Integer, TeacherResponse> teacherResponseByLessonId) {
        List<LessonResponse> lessons = response.getLessonResponses().stream()
                .sorted(Comparator.comparing(LessonResponse::getStartTime))
                .toList();
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
                    .append(String.format("📍 _%s_", lesson.getAuditorium()))
                    .append(String.format("\n\uD83E\uDDD1\u200D\uD83C\uDFEB %s\n\n", lesson.getTeacherId() == -1 ? "" :
                            teacherResponseByLessonId.get(lesson.getTeacherId()).getFullname()));
        }
        return sb.toString();
    }

    private HashMap<Integer,TeacherResponse> getTeachersForLessons(ListLessonResponse listLessonResponse) {
        HashMap<Integer,TeacherResponse> teacherResponseById = new HashMap<>();
        for(LessonResponse lesson : listLessonResponse.getLessonResponses()) {
            TeacherResponse teacherResponse = teacherFeignClient.getTeacherById(lesson.getTeacherId().toString());
            if(teacherResponse != null && teacherResponse.getFullname() != null )
                teacherResponseById.put(lesson.getTeacherId(),teacherResponse);
        }
        return teacherResponseById;
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