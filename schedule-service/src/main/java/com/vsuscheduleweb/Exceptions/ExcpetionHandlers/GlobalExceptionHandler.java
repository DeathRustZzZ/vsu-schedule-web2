package com.vsuscheduleweb.Exceptions.ExcpetionHandlers;




import com.vsuscheduleweb.Exceptions.*;
import com.vsuscheduleweb.Exceptions.Errors.AppError;
import lombok.extern.slf4j.Slf4j;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.ControllerAdvice;
import org.springframework.web.bind.annotation.ExceptionHandler;


@Slf4j
@ControllerAdvice
public class GlobalExceptionHandler {

    @ExceptionHandler({
            ResponseNotFoundException.class,
            GroupNotFoundException.class,
            TeacherNotFoundException.class
    })
    public ResponseEntity<AppError> notFoundExceptionHandler(RuntimeException ex){
        log.error(ex.getMessage());
        return ResponseEntity.status(HttpStatus.NOT_FOUND)
                .body(new AppError(ex.getMessage()));
    }

    @ExceptionHandler
    public ResponseEntity<AppError> ObjectIsPresentExceptionHandler(ObjectIsPresentException ex){
        log.error(ex.getMessage());
        return ResponseEntity.status(HttpStatus.CONFLICT)
                .body(new AppError(ex.getMessage()));
    }

    @ExceptionHandler({
            FileIsEmptyException.class,
            FileException.class,
            ParserException.class
    })
    public ResponseEntity<AppError> fileFormatExceptionHandler(RuntimeException ex){
        return ResponseEntity.status(HttpStatus.BAD_REQUEST)
                .body(new AppError(ex.getMessage()));
    }

}
