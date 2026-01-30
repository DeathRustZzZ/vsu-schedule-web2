package com.vsuscheduleweb.Exceptions;

public class ObjectIsPresentException extends RuntimeException{
    public ObjectIsPresentException(String message){
        super(message);
    }
}
