package com.vsuscheduleweb.DTO;

import lombok.*;

import java.util.List;

@Getter
@Setter
@NoArgsConstructor
@AllArgsConstructor
@Builder
public class ListGroupResponse {
    private List<GroupResponse> groupResponseList;
}
