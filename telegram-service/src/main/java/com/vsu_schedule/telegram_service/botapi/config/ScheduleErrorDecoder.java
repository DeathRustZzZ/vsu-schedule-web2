package com.vsu_schedule.telegram_service.botapi.config;

import com.vsu_schedule.telegram_service.botapi.exception.FeignClientNotFoundException;
import feign.FeignException;
import feign.Response;
import feign.RetryableException;
import feign.codec.ErrorDecoder;
import lombok.extern.slf4j.Slf4j;

@Slf4j
public class ScheduleErrorDecoder implements ErrorDecoder {
    @Override
    public Exception decode(String methodKey, Response response) {
        FeignException exception = FeignException.errorStatus(methodKey, response);
        log.error(exception.getMessage());
        int status = response.status();
        String[] responseMessageSplit = exception.getMessage().split("\"message\"");
        String[] exMessageSplit = responseMessageSplit[responseMessageSplit.length - 1].split("\"");
        String exMessage = exMessageSplit[exMessageSplit.length - 2];
        if (status == 404) {
            return new FeignClientNotFoundException(exMessage);
        }
        if (status >= 500) {
            return new RetryableException(
                    response.status(),
                    exception.getMessage(),
                    response.request().httpMethod(),
                    exception,
                    (Long)null,
                    response.request());
        }
        return exception;
    }
}