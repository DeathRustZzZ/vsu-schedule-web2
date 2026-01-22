package com.vsu_schedule.telegram_service.feign;


import com.vsu_schedule.telegram_service.dto.ListLessonResponse;
import org.springframework.cloud.openfeign.FeignClient;
import org.springframework.cloud.openfeign.FeignClientsConfiguration;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;

@FeignClient(
        contextId = "lessonClient",
        value = "${feign.client.lessons.name}",
        path = "${feign.client.lessons.path}",
        configuration = FeignClientsConfiguration.class
)
public interface LessonFeignClient {
    @GetMapping("/{groupId}/{subgroupId}")
    ListLessonResponse getLessonsByGroupAndSubgroup(@PathVariable String groupId, @PathVariable String subgroupId);
}
