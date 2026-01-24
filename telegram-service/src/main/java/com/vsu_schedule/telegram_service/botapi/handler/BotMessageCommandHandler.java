package com.vsu_schedule.telegram_service.botapi.handler;

import com.vsu_schedule.telegram_service.botapi.cache.LastMessageIdCache;
import com.vsu_schedule.telegram_service.botapi.event.TelegramActionEvent;
import com.vsu_schedule.telegram_service.botapi.cache.SendPhotoMessageIdCache;
import com.vsu_schedule.telegram_service.botapi.cache.TeacherSessionStore;
import com.vsu_schedule.telegram_service.botapi.command.HelpCommand;
import com.vsu_schedule.telegram_service.botapi.command.RegisterCommand;
import com.vsu_schedule.telegram_service.botapi.command.ScheduleCommand;
import com.vsu_schedule.telegram_service.botapi.command.StartCommand;
import com.vsu_schedule.telegram_service.botapi.service.BotMessageCommandService;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.context.ApplicationEventPublisher;
import org.springframework.stereotype.Component;
import org.telegram.telegrambots.meta.api.methods.botapimethods.BotApiMethod;
import org.telegram.telegrambots.meta.api.methods.updatingmessages.DeleteMessage;
import org.telegram.telegrambots.meta.api.objects.message.Message;


@Component
@RequiredArgsConstructor
@Slf4j
public class BotMessageCommandHandler {

    private final BotMessageCommandService commandService;

    private final TeacherSessionStore teacherSessionStore;

    private final SendPhotoMessageIdCache sendPhotoMessageIdCache;

    private final ApplicationEventPublisher applicationEventPublisher;

    private final LastMessageIdCache lastMessageIdCache;

    public BotApiMethod<?> handle(Message message) {
        String commandName = message.getText();
        if(lastMessageIdCache.get(message.getChatId()) != null){
            deleteLastMessage(message.getChatId());
        }
        if(teacherSessionStore.get(message.getFrom().getId()) != null) {
            deleteLastSendPhotoMessage(message.getChatId());
            teacherSessionStore.remove(message.getFrom().getId());
        }
        if(sendPhotoMessageIdCache.get(message.getChatId()) != null) {
            sendPhotoMessageIdCache.remove(message.getChatId());
        }
        if (commandName.equals(StartCommand.getCommandName())) {
            return commandService.handleStartCommand(message);
        }
        if (commandName.equals(HelpCommand.getCommandName())) {
            return commandService.handleHelpCommand(message);
        }
        if (commandName.equals(RegisterCommand.getCommandName())) {
            return commandService.handleRegisterCommand(message);
        }
        if(commandName.equals(ScheduleCommand.getCommandName())) {
            return commandService.handleScheduleCommand(message);
        }
        return null;
    }

    private void deleteLastMessage(Long chatId) {
        DeleteMessage deleteMessage = DeleteMessage.builder()
                .messageId(lastMessageIdCache.get(chatId))
                .chatId(chatId)
                .build();
        applicationEventPublisher.publishEvent(new TelegramActionEvent(this,deleteMessage));
    }
    private void deleteLastSendPhotoMessage(Long chatId) {
        DeleteMessage deleteMessage = DeleteMessage.builder()
                .messageId(sendPhotoMessageIdCache.get(chatId))
                .chatId(chatId)
                .build();
        applicationEventPublisher.publishEvent(new TelegramActionEvent(this, deleteMessage));
    }
}
