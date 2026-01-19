package com.vsu_schedule.telegram_service.botapi.command;

import org.telegram.telegrambots.meta.api.objects.commands.BotCommand;
import org.telegram.telegrambots.meta.api.objects.message.Message;

public class RegisterCommand extends BotCommand implements Command {

    private static final String command = "/register";
    private static final String description = "регистрация";

    public RegisterCommand() {
        super(command,description);
    }

    public String getAnswer(Message message) {
        return "Выберете свой факультет";
    }

    public static String getCommandName() {
        return command;
    }
}