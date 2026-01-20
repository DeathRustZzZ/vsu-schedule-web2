package com.vsuscheduleweb.services;

import com.vsuscheduleweb.DTO.GroupResponse;
import com.vsuscheduleweb.DTO.GroupWithSubgroupsIds;
import com.vsuscheduleweb.DTO.ListGroupResponse;
import com.vsuscheduleweb.DTO.ListGroupWithSubgroupsIds;
import com.vsuscheduleweb.Exceptions.GroupNotFoundException;
import com.vsuscheduleweb.entity.Group;
import com.vsuscheduleweb.entity.Subgroup;
import com.vsuscheduleweb.mapper.GroupMapper;
import com.vsuscheduleweb.repositories.GroupRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;

import java.util.List;

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

    public ListGroupWithSubgroupsIds getAvailableGroupsByFaculty(String faculty) {
        faculty = faculty.equals("fmiit") ? "3" : "0";
        List<Group> groupList = groupRepository.findByFaculty(faculty);
        ListGroupWithSubgroupsIds listGroupWithSubgroupsIds = new ListGroupWithSubgroupsIds();
        for(Group group : groupList) {
            GroupWithSubgroupsIds groupWithSubgroupsIds = new GroupWithSubgroupsIds();
            groupWithSubgroupsIds.setGroupId(group.getId());
            for(Subgroup subgroup : group.getSubgroups()){
                groupWithSubgroupsIds.addSubgroupId(subgroup.getId());
            }
            listGroupWithSubgroupsIds.addGroupWithSubgroupIds(groupWithSubgroupsIds);
        }
        return listGroupWithSubgroupsIds;
    }
}
