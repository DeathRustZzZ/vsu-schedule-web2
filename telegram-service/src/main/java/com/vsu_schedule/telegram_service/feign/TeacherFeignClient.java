package com.vsu_schedule.telegram_service.feign;

import com.vsu_schedule.telegram_service.dto.TeacherResponse;
import org.springframework.cloud.openfeign.FeignClient;
import org.springframework.cloud.openfeign.FeignClientsConfiguration;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;

@FeignClient(
        contextId = "teacherClient",
        value = "${feign.client.teachers.name}",
        path = "${feign.client.teachers.path}",
        configuration = FeignClientsConfiguration.class
)
public interface TeacherFeignClient {

    @GetMapping("/{id}")
    TeacherResponse getTeacherById(@PathVariable String id);
}
