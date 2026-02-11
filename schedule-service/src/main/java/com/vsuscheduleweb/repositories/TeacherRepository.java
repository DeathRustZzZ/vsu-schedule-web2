package com.vsuscheduleweb.repositories;

import com.vsuscheduleweb.entity.Teacher;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;
import org.springframework.stereotype.Repository;

import java.util.Optional;
import java.util.UUID;

@Repository
public interface TeacherRepository extends JpaRepository<Teacher,UUID> {

    Optional<Teacher> findByInitialsAndLastnameIgnoreCase(String initials, String lastname);
    Teacher findByLastname(String lastname);
    Boolean existsById(Integer id);

    Optional<Teacher> findById(Integer id);

    @Query("select t from teachers t where t.id in :ids")
    java.util.List<Teacher> findByIdIn(@Param("ids") java.util.List<Integer> ids);

    @Query("select coalesce(max(t.id), -1) from teachers t")
    Integer findMaxId();
}
