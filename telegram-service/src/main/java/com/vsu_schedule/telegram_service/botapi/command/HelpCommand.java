package com.vsu_schedule.telegram_service.botapi.command;

import org.telegram.telegrambots.meta.api.objects.Message;
import org.telegram.telegrambots.meta.api.objects.commands.BotCommand;

public class HelpCommand extends BotCommand implements Command {

    private static final String command = "/help";
    private static final String description = "Помощь";

    public HelpCommand() {
        super.setCommand(command);
        super.setDescription(description);
    }

    public String getAnswer(Message message) {
        return "Вот список доступных:\n" +
                "/help - Помощь\n" +
                "/start - Стартовая команда\n" +
                "/register - регистрация\n";
    }

    public static String getCommandName() {
        return command;
    }

}