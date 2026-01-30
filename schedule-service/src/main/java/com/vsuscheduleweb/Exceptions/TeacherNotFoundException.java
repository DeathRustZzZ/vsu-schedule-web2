package com.vsuscheduleweb.Exceptions;

public class TeacherNotFoundException extends RuntimeException {
    public TeacherNotFoundException(String message){
        super(message);
    }
}
