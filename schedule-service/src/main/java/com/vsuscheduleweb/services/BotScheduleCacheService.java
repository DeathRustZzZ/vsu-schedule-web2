package com.vsuscheduleweb.services;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.vsuscheduleweb.DTO.ListLessonResponse;
import com.vsuscheduleweb.DTO.LessonResponse;
import com.vsuscheduleweb.entity.Group;
import com.vsuscheduleweb.entity.Lesson;
import com.vsuscheduleweb.entity.Subgroup;
import com.vsuscheduleweb.mapper.LessonMapper;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.data.redis.core.Cursor;
import org.springframework.data.redis.core.RedisCallback;
import org.springframework.data.redis.core.StringRedisTemplate;
import org.springframework.data.redis.core.ScanOptions;
import org.springframework.stereotype.Service;

import java.time.Duration;
import java.util.*;

@Service
@RequiredArgsConstructor
@Slf4j
public class BotScheduleCacheService {
    private static final String KEY_PREFIX = "schedule:v1:";

    private final StringRedisTemplate redis;
    private final ObjectMapper objectMapper;
    private final LessonMapper lessonMapper;

    @Value("${bot.schedule-cache-ttl:PT30M}")
    private Duration ttl;

    public Optional<ListLessonResponse> get(String faculty, String groupId, String subgroupId, String weekDay) {
        String key = buildKey(faculty, groupId, subgroupId, weekDay);
        String json = redis.opsForValue().get(key);
        if (json == null || json.isBlank()) {
            return Optional.empty();
        }
        try {
            return Optional.of(objectMapper.readValue(json, ListLessonResponse.class));
        } catch (Exception e) {
            log.warn("Failed to deserialize cache entry for key {}", key, e);
            return Optional.empty();
        }
    }

    public void put(String faculty, String groupId, String subgroupId, String weekDay, ListLessonResponse response) {
        String key = buildKey(faculty, groupId, subgroupId, weekDay);
        try {
            String json = objectMapper.writeValueAsString(response);
            redis.opsForValue().set(key, json, ttl);
        } catch (JsonProcessingException e) {
            log.warn("Failed to serialize cache entry for key {}", key, e);
        }
    }

    public void evictByFaculty(String faculty) {
        String pattern = KEY_PREFIX + normalize(faculty) + ":*";
        redis.execute((RedisCallback<Void>) connection -> {
            try (Cursor<byte[]> cursor = connection.scan(ScanOptions.scanOptions().match(pattern).count(1000).build())) {
                List<byte[]> keys = new ArrayList<>();
                cursor.forEachRemaining(keys::add);
                if (!keys.isEmpty()) {
                    connection.del(keys.toArray(new byte[0][]));
                }
            } catch (Exception e) {
                log.warn("Failed to evict cache for faculty {}", faculty, e);
            }
            return null;
        });
    }

    public void rebuildForFaculty(String faculty, List<Group> groups, List<Lesson> lessons) {
        if (groups == null || lessons == null) {
            return;
        }

        evictByFaculty(faculty);

        Map<String, List<LessonResponse>> commonByGroupWeekday = new HashMap<>();
        Map<String, List<LessonResponse>> subgroupByWeekday = new HashMap<>();

        for (Lesson lesson : lessons) {
            if (!Objects.equals(faculty, lesson.getFaculty())) {
                continue;
            }
            String weekDay = safe(lesson.getWeekDay());
            LessonResponse response = lessonMapper.entityToResponse(lesson);

            String subgroupId = safe(lesson.getSubgroupId());
            String groupId = safe(lesson.getGroupId());
            if (!groupId.isBlank() && subgroupId.isBlank()) {
                String key = groupId + "|" + weekDay;
                commonByGroupWeekday.computeIfAbsent(key, k -> new ArrayList<>()).add(response);
            }

            if (!subgroupId.isBlank()) {
                String key = subgroupId + "|" + weekDay;
                subgroupByWeekday.computeIfAbsent(key, k -> new ArrayList<>()).add(response);
            }
        }

        for (Group group : groups) {
            String groupId = safe(group.getId());
            if (groupId.isBlank()) {
                continue;
            }

            Set<String> subgroupIds = new HashSet<>();
            for (Subgroup subgroup : group.getSubgroups()) {
                String subgroupId = safe(subgroup.getId());
                if (!subgroupId.isBlank()) {
                    subgroupIds.add(subgroupId);
                }
            }

            if (subgroupIds.isEmpty()) {
                continue;
            }

            Set<String> weekDays = new HashSet<>();
            for (String key : commonByGroupWeekday.keySet()) {
                if (key.startsWith(groupId + "|")) {
                    weekDays.add(key.substring(groupId.length() + 1));
                }
            }
            for (String subgroupId : subgroupIds) {
                for (String key : subgroupByWeekday.keySet()) {
                    if (key.startsWith(subgroupId + "|")) {
                        weekDays.add(key.substring(subgroupId.length() + 1));
                    }
                }
            }

            for (String subgroupId : subgroupIds) {
                for (String weekDay : weekDays) {
                    List<LessonResponse> result = new ArrayList<>();
                    List<LessonResponse> common = commonByGroupWeekday.get(groupId + "|" + weekDay);
                    if (common != null) {
                        result.addAll(common);
                    }
                    List<LessonResponse> subgroupOnly = subgroupByWeekday.get(subgroupId + "|" + weekDay);
                    if (subgroupOnly != null) {
                        result.addAll(subgroupOnly);
                    }
                    if (!result.isEmpty()) {
                        put(faculty, groupId, subgroupId, weekDay, new ListLessonResponse(result));
                    }
                }
            }
        }
    }

    private static String buildKey(String faculty, String groupId, String subgroupId, String weekDay) {
        return KEY_PREFIX + normalize(faculty) + ":" + normalize(groupId) + ":" + normalize(subgroupId) + ":" + normalize(weekDay);
    }

    private static String normalize(String value) {
        return safe(value).trim();
    }

    private static String safe(String value) {
        return value == null ? "" : value;
    }
}
