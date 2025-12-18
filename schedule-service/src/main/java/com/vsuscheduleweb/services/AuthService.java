package com.vsuscheduleweb.services;


import com.vsuscheduleweb.DTO.AppUserDto;
import com.vsuscheduleweb.DTO.AuthReqDto;
import com.vsuscheduleweb.DTO.AuthResponse;
import com.vsuscheduleweb.Exceptions.ObjectIsPresentException;
import com.vsuscheduleweb.Exceptions.ResponseNotFoundException;
import com.vsuscheduleweb.entity.AppUser;
import com.vsuscheduleweb.repositories.AppUserRepository;
import com.vsuscheduleweb.security.Token.Token;
import com.vsuscheduleweb.security.Token.TokenRepository;
import com.vsuscheduleweb.security.Token.TokenType;
import com.vsuscheduleweb.security.roles.Role;
import lombok.RequiredArgsConstructor;
import org.springframework.security.authentication.AuthenticationManager;
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken;
import org.springframework.security.crypto.password.PasswordEncoder;
import org.springframework.stereotype.Service;

import java.util.Optional;
import java.util.Set;
import java.util.UUID;

@Service
@RequiredArgsConstructor
public class AuthService {

    private final AppUserRepository userRepository;
    private final PasswordEncoder passwordEncoder;
    private final JwtService jwtService;
    private final AuthenticationManager authenticationManager;

    private final TokenRepository tokenRepository;

    public AuthResponse register(AppUserDto userDto){
        if(userRepository.findByLogin(userDto.getLogin()).isPresent()){
            throw new ObjectIsPresentException("user with name " + userDto.getLogin() + " is present.");
        }
        if(userRepository.findByEmail(userDto.getEmail()).isPresent()) {
            throw new ObjectIsPresentException("user with email " + userDto.getEmail() + " is present.");
        }
        var user = new AppUser()
                .setEmail(userDto.getEmail())
                .setName(userDto.getName())
                .setLogin(userDto.getLogin())
                .setRoles(Set.of(Role.USER))
                .setId(UUID.randomUUID())
                .setPassword(passwordEncoder.encode(userDto.getPassword()))
                .setName(userDto.getName())
                .setLastname(userDto.getLastname());
        var savedUser = userRepository.save(user);
        var jwtToken = jwtService.generateToken(user);
        saveUserToken(savedUser, jwtToken);
        return new AuthResponse().setToken(jwtToken);


    }
    private Token saveUserToken(AppUser user, String jwtToken) {
        var token = new Token()
                .setId(UUID.randomUUID() )
                .setUser(user)
                .setToken(jwtToken)
                .setTokenType(TokenType.BEARER)
                .setExpired(false)
                .setRevoked(false);
        tokenRepository.save(token);
        return token;
    }

    private void revokeAllUserTokens(AppUser user) {
        var validUserTokens = tokenRepository.findAllValidTokenByUser(user.getId());
        if (validUserTokens.isEmpty())
            return;
        validUserTokens.forEach(token -> {
            token.setExpired(true);
            token.setRevoked(true);
        });
        tokenRepository.saveAll(validUserTokens);
    }

    public AuthResponse auth(AuthReqDto req){
        authenticationManager.authenticate(
                new UsernamePasswordAuthenticationToken(
                        req.getLogin(),
                        req.getPassword()
                )
        );
        Optional<AppUser> opt_user = userRepository.findByLogin(req.getLogin());
        if(!opt_user.isPresent()){
            throw new ResponseNotFoundException("user with name " + req.getLogin() + " not found.");
        }else {
            var jwtToken = jwtService.generateToken(opt_user.get());
            var token = saveUserToken(opt_user.get(), jwtToken);
            opt_user.get().addToken(token);
            userRepository.save(opt_user.get());
            return new AuthResponse().setToken(jwtToken);
        }

    }
}
