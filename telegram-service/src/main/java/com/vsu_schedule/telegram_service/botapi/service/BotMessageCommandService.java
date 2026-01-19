package com.vsu_schedule.telegram_service.botapi.service;


import com.vsu_schedule.telegram_service.botapi.callback_query_types.FacultyCallbackQueryTypes;
import com.vsu_schedule.telegram_service.botapi.callback_query_types.ResetRegistrationCallbackQueryTypes;
import com.vsu_schedule.telegram_service.botapi.command.HelpCommand;
import com.vsu_schedule.telegram_service.botapi.command.RegisterCommand;
import com.vsu_schedule.telegram_service.botapi.command.StartCommand;
import com.vsu_schedule.telegram_service.entity.BotUser;
import com.vsu_schedule.telegram_service.repository.BotUserRepository;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Component;
import org.telegram.telegrambots.meta.api.methods.botapimethods.BotApiMethod;
import org.telegram.telegrambots.meta.api.methods.send.SendMessage;
import org.telegram.telegrambots.meta.api.methods.updatingmessages.EditMessageReplyMarkup;
import org.telegram.telegrambots.meta.api.objects.message.Message;
import org.telegram.telegrambots.meta.api.objects.replykeyboard.InlineKeyboardMarkup;
import org.telegram.telegrambots.meta.api.objects.replykeyboard.buttons.InlineKeyboardButton;
import org.telegram.telegrambots.meta.api.objects.replykeyboard.buttons.InlineKeyboardRow;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

@Component
@RequiredArgsConstructor
@Slf4j
public class BotMessageCommandService {

    private final BotUserRepository botUserRepository;

    public BotApiMethod<?> handleStartCommand(Message message){
        String chatId = message.getChatId().toString();
        return new SendMessage(chatId,new StartCommand().getAnswer(message));
    }

    public BotApiMethod<?> handleHelpCommand(Message message) {
        return new SendMessage(message.getChatId().toString(),new HelpCommand().getAnswer(message));
    }

    public BotApiMethod<?> handleRegisterCommand(Message message){
        String chatId = message.getChatId().toString();
        if(!botUserRepository.existsByTelegramId(message.getFrom().getId())){
            botUserRepository.save(new BotUser()
                    .setTelegramId(message.getFrom().getId())
                    .setChatId(message.getChatId())
                    .setUsername(message.getFrom().getUserName()));
            SendMessage sendMessage =  new SendMessage(chatId,new RegisterCommand().getAnswer(message));
            sendMessage.setReplyMarkup(getFacultiesInlineKeyboard());
            return sendMessage;

        }else {
            SendMessage sendMessage =  new SendMessage(chatId,"Вы уже были зарегистрированы. Хотите пройти регистрацию заново?");
            sendMessage.setReplyMarkup(getAnswersResetRegistrationInlineKeyBoard());
            return sendMessage;
        }
    }

    private InlineKeyboardMarkup getFacultiesInlineKeyboard() {
        List<InlineKeyboardButton> buttonList = new ArrayList<>();
        Collections.addAll(buttonList,
                InlineKeyboardButton.builder()
                        .callbackData(FacultyCallbackQueryTypes.FMIIT.toString())
                        .text("ФМиИТ")
                        .build()
        );
        InlineKeyboardRow inlineKeyboardRow = new InlineKeyboardRow(buttonList);
        return InlineKeyboardMarkup
                .builder()
                .keyboardRow(inlineKeyboardRow)
                .build();

    }
    private InlineKeyboardMarkup getAnswersResetRegistrationInlineKeyBoard() {
        List<InlineKeyboardButton> buttonList = new ArrayList<>();
        Collections.addAll(buttonList,
                InlineKeyboardButton.builder()
                        .callbackData(ResetRegistrationCallbackQueryTypes.YES.toString())
                        .text("Да")
                        .build(),
                InlineKeyboardButton.builder()
                        .callbackData(ResetRegistrationCallbackQueryTypes.NO.toString())
                        .text("Нет")
                        .build()
        );
        InlineKeyboardRow inlineKeyboardRow = new InlineKeyboardRow(buttonList);
        return InlineKeyboardMarkup
                .builder()
                .keyboardRow(inlineKeyboardRow)
                .build();
    }
}
