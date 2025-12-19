package com.vsu_schedule.telegram_service.repository;

import com.vsu_schedule.telegram_service.entity.Student;
import org.springframework.data.jpa.repository.JpaRepository;

public interface StudentRepository extends JpaRepository<Student, Long> {
}
