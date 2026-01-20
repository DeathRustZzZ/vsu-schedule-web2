package com.vsu_schedule.telegram_service.dto;

import lombok.*;

import java.util.ArrayList;
import java.util.List;

@Setter
@Getter
@NoArgsConstructor
@EqualsAndHashCode
@ToString
@Builder
public class ListGroupWithSubgroupsIds {
    private final List<GroupWithSubgroupsIds> listGroupWithSubgroupsIds = new ArrayList<>();

    public void addGroupWithSubgroupIds(GroupWithSubgroupsIds groupWithSubgroupsIds){
        listGroupWithSubgroupsIds.add(groupWithSubgroupsIds);
    }
}
