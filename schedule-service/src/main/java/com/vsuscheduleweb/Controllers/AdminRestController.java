package com.vsuscheduleweb.Controllers;


import com.vsuscheduleweb.services.ScheduleService;
import jakarta.servlet.http.HttpServletRequest;
import lombok.RequiredArgsConstructor;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;
import org.springframework.web.multipart.MultipartFile;

@RestController()
@RequestMapping("/api/v1/schedule")
@RequiredArgsConstructor
public class AdminRestController {

    private final ScheduleService scheduleService;

    @PostMapping("/uploadFile")
    @ResponseStatus(HttpStatus.OK)
    public void uploadSchedule(@RequestPart MultipartFile file,
                               @RequestParam("f") String fac) {
        scheduleService.uploadSchedule(file,fac);
    }
}
