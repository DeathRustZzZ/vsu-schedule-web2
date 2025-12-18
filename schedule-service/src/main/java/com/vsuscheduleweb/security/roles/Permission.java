package com.vsuscheduleweb.security.roles;

import lombok.Getter;

@Getter
public enum Permission {

    EXAMPLE_PERMISSION("permission:example");


    public final String permission;

    Permission(String permission){
        this.permission = permission;
    }



}
