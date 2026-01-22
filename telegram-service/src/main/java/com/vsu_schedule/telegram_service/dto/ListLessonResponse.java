package com.vsu_schedule.telegram_service.dto;

import lombok.*;

import java.util.List;

@Getter
@Setter
@NoArgsConstructor
@AllArgsConstructor
@Builder
public class ListLessonResponse {
    List<LessonResponse> lessonResponses;
}
