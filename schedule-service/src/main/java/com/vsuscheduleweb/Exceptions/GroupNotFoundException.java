package com.vsuscheduleweb.Exceptions;

import com.vsuscheduleweb.DTO.GroupResponse;

public class GroupNotFoundException extends RuntimeException{
    public GroupNotFoundException(String message){
        super(message);
    }
}
