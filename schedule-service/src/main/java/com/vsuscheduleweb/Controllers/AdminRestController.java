package com.vsuscheduleweb.Controllers;


import com.vsuscheduleweb.services.ScheduleService;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.dao.DataIntegrityViolationException;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;
import org.springframework.web.multipart.MultipartFile;

import java.util.Map;

@RestController()
@RequestMapping("/api/v1/schedule")
@RequiredArgsConstructor
@Slf4j
public class AdminRestController {

    private final ScheduleService scheduleService;

    @PostMapping("/uploadFile")
    public ResponseEntity<?> uploadSchedule(@RequestPart MultipartFile file,
                                            @RequestParam("f") String fac) {
        try {
            log.info("📁 Uploading schedule file: {}, size: {} bytes, faculty: {}",
                    file.getOriginalFilename(), file.getSize(), fac);
            scheduleService.uploadSchedule(file, fac);
            log.info("✅ Schedule uploaded successfully");
            return ResponseEntity.ok(Map.of("message", "Schedule uploaded successfully"));
        } catch (DataIntegrityViolationException e) {
            String errorMsg = e.getMostSpecificCause().getMessage();
            log.error("💥 Database constraint violation: {}", errorMsg, e);
            return ResponseEntity.status(HttpStatus.INTERNAL_SERVER_ERROR)
                    .body(Map.of("error", "Database error: " + errorMsg));
        } catch (Exception e) {
            log.error("💥 Failed to upload schedule", e);
            return ResponseEntity.status(HttpStatus.INTERNAL_SERVER_ERROR)
                    .body(Map.of("error", e.getMessage(), "type", e.getClass().getSimpleName()));
        }
    }
}
