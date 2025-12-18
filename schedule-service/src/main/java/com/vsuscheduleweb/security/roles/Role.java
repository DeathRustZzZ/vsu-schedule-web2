package com.vsuscheduleweb.security.roles;

import lombok.Getter;

import java.util.Set;



@Getter
public enum Role {
    USER(Set.of(Permission.EXAMPLE_PERMISSION)),
    ADMIN(Set.of(Permission.EXAMPLE_PERMISSION));
    private final Set<Permission> permissions;

    Role(Set<Permission> permissions){
        this.permissions = permissions;
    }


}
