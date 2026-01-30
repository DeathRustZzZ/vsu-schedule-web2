package com.vsuscheduleweb.DTO;

import lombok.*;

import java.util.ArrayList;
import java.util.List;

@Setter
@Getter
@AllArgsConstructor
@NoArgsConstructor
@EqualsAndHashCode
@ToString
@Builder
public class GroupWithSubgroupsIds {
    private String groupId;
    private final List<String> subgroupIds = new ArrayList<>();

    public void addSubgroupId(String id){
        subgroupIds.add(id);
    }

}
