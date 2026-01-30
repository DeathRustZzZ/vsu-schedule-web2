package com.vsu_schedule.telegram_service.botapi.callback_query_types;

import lombok.Getter;
import lombok.RequiredArgsConstructor;

@RequiredArgsConstructor
@Getter
public enum TeacherDescriptionCallBackQueryTypes {
    BACK("teacher.description.back","назад");
    private final String type;
    private final String text;
}
