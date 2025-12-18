package com.vsuscheduleweb.Controllers;

import com.vsuscheduleweb.DTO.AppUserDto;
import com.vsuscheduleweb.services.AuthService;
import com.vsuscheduleweb.DTO.AuthResponse;
import com.vsuscheduleweb.DTO.AuthReqDto;
import com.vsuscheduleweb.services.LogoutService;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import lombok.RequiredArgsConstructor;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.security.core.Authentication;
import org.springframework.web.bind.annotation.*;

@RestController
@RequestMapping("/rest/auth")
@RequiredArgsConstructor
public class AuthController {

    private final AuthService authService;

    private final LogoutService logoutService;

    //TODO for tests, in realised version will be deleted
    @PostMapping("/register")
    public ResponseEntity<AuthResponse> register(@RequestBody AppUserDto req){
        return new ResponseEntity<>(authService.register(req), HttpStatus.OK);
    }
    @PostMapping("/authenticate")
    public ResponseEntity<AuthResponse> auth(@RequestBody AuthReqDto req){
        return new ResponseEntity<>(authService.auth(req), HttpStatus.OK);
    }

    @GetMapping("/logout")
    public HttpStatus logout(
            HttpServletRequest request,
            HttpServletResponse response,
            Authentication authentication
    ){
        logoutService.logout(request,response,authentication);
        return HttpStatus.OK;
    }

}
