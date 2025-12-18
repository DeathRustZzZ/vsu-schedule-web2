package com.vsuscheduleweb.Exceptions;



public class ResponseNotFoundException extends RuntimeException{
    public ResponseNotFoundException(String message){
        super(message);
    }
}
