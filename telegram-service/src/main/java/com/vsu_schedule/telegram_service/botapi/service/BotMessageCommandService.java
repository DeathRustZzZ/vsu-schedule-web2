package com.vsu_schedule.telegram_service.botapi.service;


import com.vsu_schedule.telegram_service.botapi.callback_query_types.FacultyCallbackQueryTypes;
import com.vsu_schedule.telegram_service.botapi.command.HelpCommand;
import com.vsu_schedule.telegram_service.botapi.command.RegisterCommand;
import com.vsu_schedule.telegram_service.botapi.command.StartCommand;
import com.vsu_schedule.telegram_service.entity.BotUser;
import com.vsu_schedule.telegram_service.repository.BotUserRepository;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Component;
import org.telegram.telegrambots.meta.api.methods.BotApiMethod;
import org.telegram.telegrambots.meta.api.methods.send.SendMessage;
import org.telegram.telegrambots.meta.api.objects.Message;
import org.telegram.telegrambots.meta.api.objects.replykeyboard.InlineKeyboardMarkup;
import org.telegram.telegrambots.meta.api.objects.replykeyboard.buttons.InlineKeyboardButton;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

@Component
@RequiredArgsConstructor
@Slf4j
public class BotMessageCommandService {

    private final BotUserRepository botUserRepository;

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
        if(!botUserRepository.existsByTelegramId(message.getFrom().getId())){
            botUserRepository.save(new BotUser()
                    .setTelegramId(message.getFrom().getId())
                    .setChatId(message.getChatId())
                    .setUsername(message.getFrom().getUserName()));
            sendMessage.setText(new RegisterCommand().getAnswer(message));
            sendMessage.setReplyMarkup(getFacultiesInlineKeyboard());
        }else {
            sendMessage.setText("Вы уже были зарегистрированы.");
        }
        return sendMessage;
    }

    private InlineKeyboardMarkup getFacultiesInlineKeyboard() {
        List<InlineKeyboardButton> buttonList = new ArrayList<>();
        Collections.addAll(buttonList,
                InlineKeyboardButton.builder()
                        .callbackData(FacultyCallbackQueryTypes.FMIIT.toString())
                        .text("ФМиИТ")
                        .build()
        );
        return InlineKeyboardMarkup
                .builder()
                .keyboardRow(buttonList)
                .build();

    }
}
