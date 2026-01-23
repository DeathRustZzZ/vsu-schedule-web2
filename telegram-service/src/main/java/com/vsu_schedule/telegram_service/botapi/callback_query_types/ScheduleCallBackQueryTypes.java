package com.vsu_schedule.telegram_service.botapi.callback_query_types;

import lombok.Getter;
import lombok.RequiredArgsConstructor;

@RequiredArgsConstructor
@Getter
public enum ScheduleCallBackQueryTypes {
    BACK("schedule.back","назад"),
    TEACHER_LIST("schedule.teacher.list", "[ 👨‍🏫 О преподавателях ]");
    private final String type;
    private final String text;
}