package com.vsu_schedule.telegram_service.botapi.exception.handler;


import com.vsu_schedule.telegram_service.botapi.exception.FeignClientNotFoundException;
import feign.RetryableException;
import lombok.extern.slf4j.Slf4j;
import org.springframework.web.bind.annotation.ControllerAdvice;
import org.springframework.web.bind.annotation.ExceptionHandler;

@ControllerAdvice
@Slf4j
public class GlobalExceptionHandler {


    @ExceptionHandler(FeignClientNotFoundException.class)
    public void feignClientNotFoundException(RuntimeException e) {
        log.error(e.getMessage());
    }

    @ExceptionHandler(RetryableException.class)
    public void feignClientRetryableException(RuntimeException e){
        log.error(e.getMessage());
    }
}
