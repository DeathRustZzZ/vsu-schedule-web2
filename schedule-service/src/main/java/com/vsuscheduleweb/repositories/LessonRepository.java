package com.vsuscheduleweb.repositories;

import com.vsuscheduleweb.entity.Lesson;
import jakarta.transaction.Transactional;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Modifying;
import org.springframework.data.jpa.repository.Query;

import java.util.UUID;

public interface LessonRepository extends JpaRepository<Lesson, UUID> {

    @Transactional
    @Modifying
    @Query("delete from lessons l where l.facult = ?1")
    void deleteAllWhereFacultEquals(String facult);

}
