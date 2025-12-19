package com.vsu_schedule.telegram_service.botapi.command;

import org.telegram.telegrambots.meta.api.objects.Message;
import org.telegram.telegrambots.meta.api.objects.commands.BotCommand;

public class StartCommand extends BotCommand implements Command {

    private static final String command = "/start";
    private static final String description = "Начальная команда";

    public StartCommand() {
        super.setCommand(command);
        super.setDescription(description);
    }

    public String getAnswer(Message message) {
        return "Здравствуйте,"+message.getFrom().getFirstName()+"!\n"
                + "Выберете команду в меню.";
    }

    public static String getCommandName(){
        return command;
    }
}
