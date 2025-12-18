package com.vsuscheduleweb.DTO;

import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;

@Data
@AllArgsConstructor
@NoArgsConstructor
public class AppUserDto {
    private String name;
    private String lastname;
    private String email;
    private String login;
    private String password;

}
