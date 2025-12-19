package com.vsu_schedule.telegram_service.botapi.service;


import com.vsu_schedule.telegram_service.botapi.command.HelpCommand;
import com.vsu_schedule.telegram_service.botapi.command.RegisterCommand;
import com.vsu_schedule.telegram_service.botapi.command.StartCommand;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Component;
import org.telegram.telegrambots.meta.api.methods.BotApiMethod;
import org.telegram.telegrambots.meta.api.methods.send.SendMessage;
import org.telegram.telegrambots.meta.api.objects.Message;

@Component
@RequiredArgsConstructor
@Slf4j
public class BotMessageCommandService {
    public BotApiMethod<?> handleStartCommand(Message message){
        log.info("sadasdsadasdadad");
        SendMessage sendMessage = new SendMessage();
        sendMessage.setChatId(message.getChatId());
        sendMessage.setText(new StartCommand().getAnswer(message));
        return sendMessage;
    }

    public BotApiMethod<?> handleHelpCommand(Message message) {
        SendMessage sendMessage = new SendMessage();
        sendMessage.setChatId(message.getChatId());
        sendMessage.setText(new HelpCommand().getAnswer(message));
        return sendMessage;
    }

    public BotApiMethod<?> handleRegisterCommand(Message message){
        SendMessage sendMessage = new SendMessage();
        sendMessage.setChatId(message.getChatId());
        sendMessage.setText(new RegisterCommand().getAnswer(message));
        return sendMessage;
    }
}
