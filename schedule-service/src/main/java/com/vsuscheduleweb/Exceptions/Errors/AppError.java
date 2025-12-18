package com.vsuscheduleweb.Exceptions.Errors;


import lombok.*;

@Getter
@Setter
@AllArgsConstructor
@Builder
@NoArgsConstructor
@ToString
@EqualsAndHashCode
public class AppError {
    private String message;
}
