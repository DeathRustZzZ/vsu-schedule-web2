package com.vsu_schedule.telegram_service.botapi.exception;

import lombok.*;

@Getter
@Setter
@AllArgsConstructor
@Builder
@NoArgsConstructor
@ToString
@EqualsAndHashCode
public class AppError  {

    private String message;

}