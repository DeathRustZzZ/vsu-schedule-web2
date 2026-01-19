package com.vsuscheduleweb.Controllers;

import com.vsuscheduleweb.DTO.GroupResponse;
import com.vsuscheduleweb.DTO.ListGroupResponse;
import com.vsuscheduleweb.services.GroupService;
import lombok.RequiredArgsConstructor;
import org.springframework.http.HttpStatus;
import org.springframework.web.bind.annotation.*;


@RequiredArgsConstructor
@RestController()
@RequestMapping("/api/v1/groups")
public class GroupController {

    private final GroupService groupService;

    @GetMapping
    @ResponseStatus(HttpStatus.OK)
    public ListGroupResponse getAll() {
        return groupService.getAll();
    }

    @GetMapping("/{id}")
    @ResponseStatus(HttpStatus.OK)
    public GroupResponse getById(@PathVariable String id) {
        return groupService.getById(id);
    }
}
