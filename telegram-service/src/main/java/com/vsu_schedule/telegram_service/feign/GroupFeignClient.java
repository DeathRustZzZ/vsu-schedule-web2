package com.vsu_schedule.telegram_service.feign;

import com.vsu_schedule.telegram_service.dto.ListGroupWithSubgroupsIds;
import org.springframework.cloud.openfeign.FeignClient;
import org.springframework.cloud.openfeign.FeignClientsConfiguration;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;

@FeignClient(
        value = "${feign.client.schedule.name}",
        path = "${feign.client.schedule.path}",
        configuration = FeignClientsConfiguration.class
)
public interface GroupFeignClient {
    @GetMapping("/available/{faculty}")
    ListGroupWithSubgroupsIds getAvailableGroupsByFaculty(@PathVariable("faculty") String faculty);

}
