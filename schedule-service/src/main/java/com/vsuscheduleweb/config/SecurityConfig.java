package com.vsuscheduleweb.config;

import com.vsuscheduleweb.security.JwtAuthFilter;
import lombok.RequiredArgsConstructor;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.security.authentication.AuthenticationProvider;
import org.springframework.security.config.annotation.web.builders.HttpSecurity;
import org.springframework.security.config.annotation.web.configuration.EnableWebSecurity;
import org.springframework.security.config.http.SessionCreationPolicy;
import org.springframework.security.web.SecurityFilterChain;
import org.springframework.security.web.authentication.UsernamePasswordAuthenticationFilter;


@Configuration
@EnableWebSecurity
@RequiredArgsConstructor
public class SecurityConfig {

    private final JwtAuthFilter jwtAuthFilter;
    private final AuthenticationProvider authenticationProvider;
    @Bean
    public SecurityFilterChain securityFilterChain(HttpSecurity httpSecurity) throws Exception{
         httpSecurity
                 .csrf()
                 .disable()
                 .authorizeHttpRequests()
                 .requestMatchers("/api/v1/schedule/auth/**").permitAll()
                 .requestMatchers("/api/v1/teachers").permitAll()
                 .requestMatchers("/api/v1/groups").permitAll()
                 .requestMatchers("/schedule").authenticated()
                 .requestMatchers("/schedule/css/**","/schedule/js/**","/schedule/img/**").permitAll()
                 .requestMatchers("/schedule/admin").authenticated()
                 .requestMatchers("/schedule/auth/logout").permitAll()
                 .anyRequest().authenticated()
                 .and().formLogin(form->{
                     form.loginPage("/login")
                             .defaultSuccessUrl("/vsuAdmin")
                             .loginProcessingUrl("/login")
                             .permitAll();
                 })
                 .sessionManagement()
                 .sessionCreationPolicy(SessionCreationPolicy.ALWAYS)
                 .and()
                 .authenticationProvider(authenticationProvider)
                 .addFilterBefore(jwtAuthFilter, UsernamePasswordAuthenticationFilter.class)
                 ;

         return httpSecurity.build();
    }




}
