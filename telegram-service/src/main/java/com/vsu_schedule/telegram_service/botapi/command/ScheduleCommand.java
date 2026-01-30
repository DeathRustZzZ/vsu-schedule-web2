package com.vsu_schedule.telegram_service.botapi.command;

import org.telegram.telegrambots.meta.api.objects.commands.BotCommand;
import org.telegram.telegrambots.meta.api.objects.message.Message;

public class ScheduleCommand extends BotCommand implements Command {

    private static final String command = "/schedule";
    private static final String description = "расписание";

    public ScheduleCommand() {
        super(command,description);
    }

    public String getAnswer(Message message) {
        return "Здравствуйте,"+message.getFrom().getFirstName()+"!\n"
                + "Выберете команду в меню.";
    }

    public static String getCommandName(){
        return command;
    }
}
