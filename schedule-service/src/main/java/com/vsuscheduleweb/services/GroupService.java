package com.vsuscheduleweb.services;

import com.vsuscheduleweb.DTO.GroupResponse;
import com.vsuscheduleweb.DTO.ListGroupResponse;
import com.vsuscheduleweb.Exceptions.GroupNotFoundException;
import com.vsuscheduleweb.mapper.GroupMapper;
import com.vsuscheduleweb.repositories.GroupRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;

@Service
@RequiredArgsConstructor
public class GroupService {

    private final GroupRepository groupRepository;
    private final GroupMapper groupMapper;

    public ListGroupResponse getAll() {
        return new ListGroupResponse(
                groupRepository.findAll()
                        .stream()
                        .map(groupMapper::entityToResponse)
                        .toList()
        );
    }

    public GroupResponse getById(String id){
        return groupMapper.entityToResponse(
                groupRepository.findById(id).orElseThrow(() -> new GroupNotFoundException(String.format(
                        "group with id: %s is not found.", id)))
        );
    }

    public ListGroupResponse getAvailableGroupsByFaculty(String faculty) {
        return new ListGroupResponse(
                groupRepository.findByFaculty(faculty)
                        .stream()
                        .map(groupMapper::entityToResponse)
                        .toList()
        );
    }
}
