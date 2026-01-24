package com.vsu_schedule.telegram_service.botapi.command;

import org.telegram.telegrambots.meta.api.objects.commands.BotCommand;
import org.telegram.telegrambots.meta.api.objects.message.Message;

public class StartCommand extends BotCommand implements Command {

    private static final String command = "/start";
    private static final String description = "Начальная команда";

    public StartCommand() {
        super(command,description);
    }

    public String getAnswer(Message message) {
        return String.format("👋 Здравствуйте, %s!\n\n" +
                        "Рад вас видеть. Выберите нужную команду в меню, чтобы начать работу ⬇️",
                message.getFrom().getFirstName());
    }

    public static String getCommandName(){
        return command;
    }
}
