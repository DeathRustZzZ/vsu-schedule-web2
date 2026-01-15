package com.vsuscheduleweb.DTO;

import jakarta.validation.constraints.Email;
import jakarta.validation.constraints.NotBlank;
import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;

@Data
@AllArgsConstructor
@NoArgsConstructor
public class AuthRequest {
    @NotBlank(message = "password.not-blank")
    private String password;
    @NotBlank(message = "email.not-blank")
    @Email(message = "email.invalid")
    private String login;
}
