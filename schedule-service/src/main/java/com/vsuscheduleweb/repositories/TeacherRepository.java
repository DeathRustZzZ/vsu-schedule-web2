package com.vsuscheduleweb.repositories;

import com.vsuscheduleweb.entity.Teacher;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

import java.util.Optional;
import java.util.UUID;

@Repository
public interface TeacherRepository extends JpaRepository<Teacher,UUID> {

    Optional<Teacher> findByInitialsAndLastname(String initials, String lastname);
    Teacher findByLastname(String lastname);
    Boolean existsById(Integer id);

    Optional<Teacher> findById(Integer id);

}
