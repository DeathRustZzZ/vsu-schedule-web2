package com.vsuscheduleweb.repositories;

import com.vsuscheduleweb.entity.AppUser;
import org.springframework.data.jpa.repository.JpaRepository;

import java.util.Optional;
import java.util.UUID;

public interface AppUserRepository extends JpaRepository<AppUser, UUID> {
    Optional<AppUser> findByLoginIgnoreCase(String login);

    Optional<AppUser> findByLoginIgnoreCaseOrEmailIgnoreCase(String login, String email);
}
