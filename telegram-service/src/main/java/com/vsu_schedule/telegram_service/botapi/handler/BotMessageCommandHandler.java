package com.vsu_schedule.telegram_service.botapi.handler;

import com.vsu_schedule.telegram_service.botapi.cache.SendPhotoMessageIdCache;
import com.vsu_schedule.telegram_service.botapi.cache.TeacherSessionStore;
import com.vsu_schedule.telegram_service.botapi.command.HelpCommand;
import com.vsu_schedule.telegram_service.botapi.command.RegisterCommand;
import com.vsu_schedule.telegram_service.botapi.command.ScheduleCommand;
import com.vsu_schedule.telegram_service.botapi.command.StartCommand;
import com.vsu_schedule.telegram_service.botapi.service.BotMessageCommandService;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Component;
import org.telegram.telegrambots.meta.api.methods.botapimethods.BotApiMethod;
import org.telegram.telegrambots.meta.api.objects.message.Message;


@Component
@RequiredArgsConstructor
@Slf4j
public class BotMessageCommandHandler {

    private final BotMessageCommandService commandService;

    private final TeacherSessionStore teacherSessionStore;

    private final SendPhotoMessageIdCache sendPhotoMessageIdCache;

    public BotApiMethod<?> handle(Message message) {
        String commandName = message.getText();
        if(teacherSessionStore.get(message.getFrom().getId()) != null) {
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
}
