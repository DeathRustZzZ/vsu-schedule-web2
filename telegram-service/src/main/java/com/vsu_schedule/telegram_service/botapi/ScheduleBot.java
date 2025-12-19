package com.vsu_schedule.telegram_service.botapi;


import com.vsu_schedule.telegram_service.botapi.command.HelpCommand;
import com.vsu_schedule.telegram_service.botapi.command.RegisterCommand;
import com.vsu_schedule.telegram_service.botapi.command.StartCommand;
import lombok.Getter;
import lombok.Setter;
import lombok.experimental.Accessors;
import lombok.extern.slf4j.Slf4j;
import org.telegram.telegrambots.meta.api.methods.BotApiMethod;
import org.telegram.telegrambots.meta.api.methods.commands.SetMyCommands;
import org.telegram.telegrambots.meta.api.methods.send.SendMessage;
import org.telegram.telegrambots.meta.api.methods.updates.SetWebhook;
import org.telegram.telegrambots.meta.api.objects.Update;
import org.telegram.telegrambots.meta.api.objects.commands.BotCommand;
import org.telegram.telegrambots.meta.api.objects.commands.scope.BotCommandScopeDefault;
import org.telegram.telegrambots.meta.exceptions.TelegramApiException;
import org.telegram.telegrambots.starter.SpringWebhookBot;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

@Setter
@Getter
@Accessors(chain = true)
@Slf4j
public class ScheduleBot extends SpringWebhookBot {

    private String botPath;
    private String botName;
    private String botToken;
    private TelegramFacade telegramFacade;
    private SetWebhook setWebhook;
    private List<BotCommand> botCommandList = new ArrayList<>();

    public ScheduleBot(SetWebhook setWebhook,
                       TelegramFacade telegramFacade,
                       String token,
                       String name,
                       String path)  {
        super(setWebhook,token);
        this.botToken = token;
        this.botName = name;
        this.botPath = path;
        this.telegramFacade = telegramFacade;
        Collections.addAll(botCommandList,
                new StartCommand(),
                new HelpCommand(),
                new RegisterCommand()
        );
        try {
            execute(new SetMyCommands(botCommandList, new BotCommandScopeDefault(), null));
        }catch (TelegramApiException e){
            log.error(e.getMessage());
        }
    }
    @Override
    public BotApiMethod<?> onWebhookUpdateReceived(Update req) {
        return telegramFacade.handleUpdate(req);
    }

    public void sendMessage(SendMessage message) {
        try {
            execute(message);
        } catch (TelegramApiException e) {
            log.error(e.getMessage());
        }
    }

    @Override
    public String getBotUsername() {
        return this.botName;
    }
}
