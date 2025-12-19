package com.vsu_schedule.telegram_service.botapi.command;

import org.telegram.telegrambots.meta.api.objects.Message;
import org.telegram.telegrambots.meta.api.objects.commands.BotCommand;

public class RegisterCommand extends BotCommand implements Command {

    private static final String command = "/register";
    private static final String description = "регистрация";

    public RegisterCommand() {
        super.setCommand(command);
        super.setDescription(description);
    }

    public String getAnswer(Message message) {
        return "";
    }

    public static String getCommandName() {
        return command;
    }
}