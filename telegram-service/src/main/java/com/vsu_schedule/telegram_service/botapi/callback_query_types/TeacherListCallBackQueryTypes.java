package com.vsu_schedule.telegram_service.botapi.callback_query_types;

import lombok.Getter;
import lombok.RequiredArgsConstructor;

@Getter
@RequiredArgsConstructor
public enum TeacherListCallBackQueryTypes {

    BACK("teacher.list.back", "назад");

    private final String type;
    private final String text;
}
