package com.vsuscheduleweb.services;

import com.vsuscheduleweb.entity.AppUser;
import com.vsuscheduleweb.repositories.AppUserRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.security.core.GrantedAuthority;
import org.springframework.security.core.authority.SimpleGrantedAuthority;
import org.springframework.security.core.userdetails.User;
import org.springframework.security.core.userdetails.UserDetails;
import org.springframework.security.core.userdetails.UserDetailsService;
import org.springframework.security.core.userdetails.UsernameNotFoundException;
import org.springframework.security.crypto.password.PasswordEncoder;
import org.springframework.stereotype.Service;

import java.time.LocalDate;
import java.util.List;
import java.util.Optional;
import java.util.UUID;

@Service
@RequiredArgsConstructor
public class AppUserService implements UserDetailsService {

    private final AppUserRepository appUserRepository;
    private final PasswordEncoder passwordEncoder;

    public Optional<AppUser> findByLoginOrEmail(String loginOrEmail) {
        return appUserRepository.findByLoginIgnoreCaseOrEmailIgnoreCase(loginOrEmail, loginOrEmail);
    }

    public AppUser createUser(String login, String rawPassword) {
        AppUser user = new AppUser()
                .setId(UUID.randomUUID())
                .setCreatedAt(LocalDate.now())
                .setLogin(login)
                .setPasswordHash(passwordEncoder.encode(rawPassword))
                .setStatus("ACTIVE");
        return appUserRepository.save(user);
    }

    public boolean matchesPassword(String rawPassword, String passwordHash) {
        return passwordEncoder.matches(rawPassword, passwordHash);
    }

    @Override
    public UserDetails loadUserByUsername(String username) throws UsernameNotFoundException {
        AppUser user = findByLoginOrEmail(username)
                .orElseThrow(() -> new UsernameNotFoundException("User not found: " + username));

        List<GrantedAuthority> authorities = List.of(new SimpleGrantedAuthority("ROLE_ADMIN"));
        return new User(user.getLogin(), user.getPasswordHash(), authorities);
    }
}
