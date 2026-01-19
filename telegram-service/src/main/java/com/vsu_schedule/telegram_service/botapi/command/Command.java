package com.vsu_schedule.telegram_service.botapi.command;


import org.telegram.telegrambots.meta.api.objects.message.Message;

public interface Command {
    default String getAnswerTextForUnregisteredUsers(String name) {
        return "Здравствуйте,"+name+" ! Вы еще не зарегистрированы, " +
                "что бы зарегистрироваться отправьте \n/register.";
    }

    String getAnswer(Message message);
}
