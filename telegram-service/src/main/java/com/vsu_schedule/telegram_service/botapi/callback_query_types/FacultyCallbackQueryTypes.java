package com.vsu_schedule.telegram_service.botapi.callback_query_types;

import lombok.RequiredArgsConstructor;

@RequiredArgsConstructor
public enum FacultyCallbackQueryTypes {
    FMIIT("faculty.fmiit");
    private final String type;

    @Override
    public String toString() {
        return type;
    }
}
