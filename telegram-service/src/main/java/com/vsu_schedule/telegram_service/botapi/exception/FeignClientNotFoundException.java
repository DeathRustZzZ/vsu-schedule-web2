package com.vsu_schedule.telegram_service.botapi.exception;

public class FeignClientNotFoundException extends RuntimeException{
    public FeignClientNotFoundException(String m) {
        super(m);
    }
}
