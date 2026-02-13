package com.vsuscheduleweb.repositories;

import com.vsuscheduleweb.entity.Lesson;
import feign.Param;
import jakarta.transaction.Transactional;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Modifying;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.jpa.repository.EntityGraph;

import java.util.List;
import java.util.UUID;

public interface LessonRepository extends JpaRepository<Lesson, UUID> {

    @Transactional
    @Modifying
    @Query("delete from lessons l where l.faculty = ?1")
    void deleteAllWhereFacultyEquals(String faculty);

    @EntityGraph(attributePaths = "teacher")
    List<Lesson> findByGroupIdOrSubgroupId(String groupId, String subgroupId);

    List<Lesson> findByFaculty(String faculty);

    @EntityGraph(attributePaths = "teacher")
    @Query("SELECT l FROM lessons l WHERE (l.groupId = :groupId OR l.subgroupId = :subgroupId) AND l.weekDay = :weekDay")
    List<Lesson> findByGroupOrSubgroupAndWeekDay(
            @Param("groupId") String groupId,
            @Param("subgroupId") String subgroupId,
            @Param("weekDay") String weekDay
    );

    @Query("SELECT l FROM lessons l WHERE l.faculty = :faculty AND (l.groupId = :groupId OR l.subgroupId = :subgroupId) AND l.weekDay = :weekDay")
    @EntityGraph(attributePaths = "teacher")
    List<Lesson> findByFacultyAndGroupOrSubgroupAndWeekDay(
            @Param("faculty") String faculty,
            @Param("groupId") String groupId,
            @Param("subgroupId") String subgroupId,
            @Param("weekDay") String weekDay
    );

    @Query("SELECT l FROM lessons l WHERE l.faculty = :faculty AND (l.groupId = :groupId OR l.subgroupId = :subgroupId)")
    @EntityGraph(attributePaths = "teacher")
    List<Lesson> findByFacultyAndGroupOrSubgroup(
            @Param("faculty") String faculty,
            @Param("groupId") String groupId,
            @Param("subgroupId") String subgroupId
    );
}
