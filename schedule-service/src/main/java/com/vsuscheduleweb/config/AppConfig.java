package com.vsuscheduleweb.config;


import com.vsuscheduleweb.entity.Teacher;
import com.vsuscheduleweb.repositories.AppUserRepository;
import com.vsuscheduleweb.repositories.TeacherRepository;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.boot.CommandLineRunner;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.security.authentication.AuthenticationManager;
import org.springframework.security.authentication.AuthenticationProvider;
import org.springframework.security.authentication.dao.DaoAuthenticationProvider;
import org.springframework.security.config.annotation.authentication.configuration.AuthenticationConfiguration;
import org.springframework.security.core.userdetails.UserDetailsService;
import org.springframework.security.core.userdetails.UsernameNotFoundException;
import org.springframework.security.crypto.bcrypt.BCryptPasswordEncoder;
import org.springframework.security.crypto.password.PasswordEncoder;
import java.io.*;


@Configuration
@RequiredArgsConstructor
@Slf4j
public class AppConfig {
    private final AppUserRepository appUserRepository;

    private final TeacherRepository teacherRepository;
    @Bean
    public UserDetailsService getUserDetails(){
        return username -> appUserRepository.findByLogin(username)
                .orElseThrow(() -> new UsernameNotFoundException("user with name "+ username +"is not found."));
    }

    @Bean
    public AuthenticationProvider authenticationProvider(){
        DaoAuthenticationProvider authenticationProvider = new DaoAuthenticationProvider();
        authenticationProvider.setUserDetailsService(getUserDetails());
        authenticationProvider.setPasswordEncoder(passwordEncoder());
        return authenticationProvider;
    }

    @Bean
    public AuthenticationManager authenticationManager(AuthenticationConfiguration configuration) throws Exception{
        return configuration.getAuthenticationManager();
    }


    @Bean
    public PasswordEncoder passwordEncoder(){
        return new BCryptPasswordEncoder();
    }

    @Bean
    public CommandLineRunner loadTeachers(TeacherRepository teacherRepository){
        return args ->{
            if(!teacherRepository.existsById(-1)) {
                teacherRepository.save(new Teacher() //empty teacher
                        .setId(-1)
                        .setFirstname("")
                        .setLastname("")
                        .setSurname("")
                        .setInitials("")
                );
            }
        };
    }
}
