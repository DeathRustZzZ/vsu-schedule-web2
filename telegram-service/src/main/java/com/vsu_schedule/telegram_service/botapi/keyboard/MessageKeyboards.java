package com.vsu_schedule.telegram_service.botapi.keyboard;

import com.vsu_schedule.telegram_service.botapi.callback_query_types.FacultyCallbackQueryTypes;
import com.vsu_schedule.telegram_service.botapi.callback_query_types.ResetRegistrationCallbackQueryTypes;
import com.vsu_schedule.telegram_service.botapi.callback_query_types.ScheduleCallBackQueryTypes;
import com.vsu_schedule.telegram_service.botapi.callback_query_types.TeacherListCallBackQueryTypes;
import com.vsu_schedule.telegram_service.dto.*;
import org.telegram.telegrambots.meta.api.objects.replykeyboard.InlineKeyboardMarkup;
import org.telegram.telegrambots.meta.api.objects.replykeyboard.buttons.InlineKeyboardButton;
import org.telegram.telegrambots.meta.api.objects.replykeyboard.buttons.InlineKeyboardRow;

import java.util.ArrayList;
import java.util.Collections;
import java.util.Comparator;
import java.util.List;

public class MessageKeyboards {

    public static InlineKeyboardMarkup getGroupInlineKeyboard(String faculty,
                                                        ListGroupWithSubgroupsIds listGroupWithSubgroupsIds) {
        List<InlineKeyboardRow> inlineKeyboardRows = new ArrayList<>();
        for(GroupWithSubgroupsIds group : listGroupWithSubgroupsIds.getListGroupWithSubgroupsIds()) {
            for(String subgroupId : group.getSubgroupIds()) {
                inlineKeyboardRows.add(new InlineKeyboardRow(
                        InlineKeyboardButton.builder()
                                .callbackData("group." + group.getGroupId().replaceAll("\\." , "/") +"." +  subgroupId)
                                .text(group.getGroupId() + "/" +subgroupId)
                                .build()));
            }
        }

        return InlineKeyboardMarkup.builder()
                .keyboard(inlineKeyboardRows)
                .build();
    }

    public static InlineKeyboardMarkup getScheduleInlineKeyboard() {
        List<InlineKeyboardRow> keyboardRows = new ArrayList<>();
        keyboardRows.add(new InlineKeyboardRow(InlineKeyboardButton.builder()
                .callbackData(ScheduleCallBackQueryTypes.TEACHER_LIST.getType())
                .text(ScheduleCallBackQueryTypes.TEACHER_LIST.getText())
                .build()));
        keyboardRows.add(new InlineKeyboardRow(InlineKeyboardButton.builder()
                .callbackData(ScheduleCallBackQueryTypes.BACK.getType())
                .text(ScheduleCallBackQueryTypes.BACK.getText())
                .build()));
        return InlineKeyboardMarkup.builder()
                .keyboard(keyboardRows)
                .build();
    }


    public static InlineKeyboardMarkup getTeacherListInlineKeyboard(List<TeacherResponse> teacherResponseList) {
        List<InlineKeyboardRow> keyboardRows = new ArrayList<>();
        for(TeacherResponse teacherResponse : teacherResponseList) {
            keyboardRows.add(new InlineKeyboardRow(InlineKeyboardButton.builder()
                    .callbackData("teacher." + teacherResponse.getId())
                    .text(teacherResponse.getFullname())
                    .build()));
        }
        keyboardRows.add(new InlineKeyboardRow(InlineKeyboardButton.builder()
                .callbackData(TeacherListCallBackQueryTypes.BACK.getType())
                .text(TeacherListCallBackQueryTypes.BACK.getText())
                .build()));
        return InlineKeyboardMarkup
                .builder()
                .keyboard(keyboardRows)
                .build();
    }

    public static InlineKeyboardMarkup getFacultiesInlineKeyboard() {
        List<InlineKeyboardButton> buttonList = new ArrayList<>();
        Collections.addAll(buttonList,
                InlineKeyboardButton.builder()
                        .callbackData(FacultyCallbackQueryTypes.FMIIT.toString())
                        .text("ФМиИТ")
                        .build()
        );
        InlineKeyboardRow inlineKeyboardRow = new InlineKeyboardRow(buttonList);
        return InlineKeyboardMarkup
                .builder()
                .keyboardRow(inlineKeyboardRow)
                .build();

    }

    public static InlineKeyboardMarkup getAnswersResetRegistrationInlineKeyboard() {
        List<InlineKeyboardButton> buttonList = new ArrayList<>();
        Collections.addAll(buttonList,
                InlineKeyboardButton.builder()
                        .callbackData(ResetRegistrationCallbackQueryTypes.YES.toString())
                        .text("Да")
                        .build(),
                InlineKeyboardButton.builder()
                        .callbackData(ResetRegistrationCallbackQueryTypes.NO.toString())
                        .text("Нет")
                        .build()
        );
        InlineKeyboardRow inlineKeyboardRow = new InlineKeyboardRow(buttonList);
        return InlineKeyboardMarkup
                .builder()
                .keyboardRow(inlineKeyboardRow)
                .build();
    }

    public static InlineKeyboardMarkup getDayOfWeekSelectInlineKeyboard(ListLessonResponse lessons) {
        List<String> weekDays = new ArrayList<>();
        List<InlineKeyboardRow> keyboardRows = new ArrayList<>();
        for(LessonResponse lessonResponse : lessons.getLessonResponses()) {
            if(!weekDays.contains(lessonResponse.getWeekDay())) {
                weekDays.add(lessonResponse.getWeekDay());
            }
        }
        sortDaysOfWeek(weekDays);
        for(String weekDay : weekDays) {
            keyboardRows.add(new InlineKeyboardRow(
                    InlineKeyboardButton.builder()
                            .text(weekDay)
                            .callbackData("weekDay." + weekDay).build()
            ));
        }
        return InlineKeyboardMarkup.builder()
                .keyboard(keyboardRows)
                .build();
    }
    private static void sortDaysOfWeek(List<String> days) {
        List<String> weekOrder = List.of(
                "ПОНЕДЕЛЬНИК", "ВТОРНИК", "СРЕДА", "ЧЕТВЕРГ", "ПЯТНИЦА", "СУББОТА", "ВОСКРЕСЕНЬЕ"
        );

        days.sort(Comparator.comparingInt(day -> weekOrder.indexOf(day.toUpperCase())));
    }
}
