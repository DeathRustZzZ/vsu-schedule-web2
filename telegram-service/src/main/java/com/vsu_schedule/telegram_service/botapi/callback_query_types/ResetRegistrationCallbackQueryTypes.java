package com.vsu_schedule.telegram_service.botapi.callback_query_types;

import lombok.RequiredArgsConstructor;

@RequiredArgsConstructor
public enum ResetRegistrationCallbackQueryTypes {
    YES("reset.registration.btn.yes"),
    NO("reset.registration.btn.no");

    private final String type;
    @Override
    public String toString() {
        return type;
    }
}
