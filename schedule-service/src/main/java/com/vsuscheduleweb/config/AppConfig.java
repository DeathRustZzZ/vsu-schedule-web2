package com.vsuscheduleweb.config;


import com.fasterxml.jackson.databind.ObjectMapper;

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
import java.util.Optional;

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
    public CommandLineRunner teacherLoader(TeacherRepository teacherRepository){
        return args ->{
            String pathTeachersDir = System.getProperty("user.dir") + "\\schedule-service\\src\\main\\scriptspython\\teachers";
            Process p =  Runtime.getRuntime().exec("python " + System.getProperty("user.dir") + "\\schedule-service\\src\\main\\scriptspython\\teacherParser.py");
            InputStream stdout = p.getInputStream();
            InputStream stderr = p.getErrorStream();
            InputStreamReader isr = new InputStreamReader(stdout);
            InputStreamReader isrerr = new InputStreamReader(stderr);
            BufferedReader br = new BufferedReader(isr);
            BufferedReader brerr = new BufferedReader(isrerr);
            String line = null;
            while ((line = br.readLine()) != null)
                if(line.equals("1")){
                    log.info("[+] teachers have been parsed!");
                    p.destroyForcibly();
                }

            while ((line = brerr.readLine()) != null)
                if(!line.isEmpty()) log.error("[-] Error teachers have not been parsed!" + "\n" + line );


            File dir = new File(pathTeachersDir);
            File[] arrFiles = dir.listFiles();
            ObjectMapper objectMapper = new ObjectMapper();
            for(File file : arrFiles){
                Teacher teacher = objectMapper.readValue(file, Teacher.class);
                Optional<Teacher> opt_teacher = teacherRepository.findById(teacher.getId());
                if(!opt_teacher.isPresent())
                    teacherRepository.save(teacher);

            }
            teacherRepository.save(new Teacher() //empty teacher opt.
                    .setId(-1)
                    .setFirstname("")
                    .setLastname("")
                    .setSurname("")
                    .setInitials("")
            );
            log.info("[+] teachers have been saved!");

        };
    }
}
