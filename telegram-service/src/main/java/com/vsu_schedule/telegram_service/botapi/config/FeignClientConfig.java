package com.vsu_schedule.telegram_service.botapi.config;

import feign.Retryer;
import feign.codec.ErrorDecoder;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

import static com.vsu_schedule.telegram_service.util.RetryerSettings.*;

@Configuration
public class FeignClientConfig {
    @Bean
    public ErrorDecoder errorDecoder() {
        return new ScheduleErrorDecoder();
    }

    @Bean
    public Retryer retryer() {
        return new Retryer.Default(RETRYER_PERIOD, RETRYER_MAX_PERIOD, RETRYER_MAX_ATTEMPTS);
    }
}