package com.vsuscheduleweb.config;


import com.vsuscheduleweb.entity.Teacher;
import com.vsuscheduleweb.repositories.TeacherRepository;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.boot.CommandLineRunner;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.security.crypto.bcrypt.BCryptPasswordEncoder;
import org.springframework.security.crypto.password.PasswordEncoder;


@Configuration
@RequiredArgsConstructor
@Slf4j
public class AppConfig {
    private final TeacherRepository teacherRepository;


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
