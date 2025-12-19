package com.vsu_schedule.telegram_service.controller;

import com.vsu_schedule.telegram_service.botapi.ScheduleBot;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestMethod;
import org.springframework.web.bind.annotation.RestController;
import org.telegram.telegrambots.meta.api.methods.BotApiMethod;
import org.telegram.telegrambots.meta.api.objects.Update;

@RestController
@RequiredArgsConstructor
@Slf4j
public class WebHookController {

    private final ScheduleBot bot;

    @RequestMapping(value = "/bots/index.php", method = RequestMethod.POST)
    public BotApiMethod<?> onUpdateReceived(@RequestBody Update update) {
        log.info("mesad");
        return bot.onWebhookUpdateReceived(update);
    }
}
