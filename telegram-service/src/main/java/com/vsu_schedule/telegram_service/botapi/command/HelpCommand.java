package com.vsu_schedule.telegram_service.botapi.command;

import org.telegram.telegrambots.meta.api.objects.commands.BotCommand;
import org.telegram.telegrambots.meta.api.objects.message.Message;

public class HelpCommand extends BotCommand implements Command {

    private static final String command = "/help";
    private static final String description = "Помощь";

    public HelpCommand() {
        super(command,description);
    }

    public String getAnswer(Message message) {
        return "Вот список доступных:\n" +
                "/help - Помощь\n" +
                "/start - Стартовая команда\n" +
                "/register - Регистрация\n" +
                "/schedule - Расписание\n";

    }

    public static String getCommandName() {
        return command;
    }

}