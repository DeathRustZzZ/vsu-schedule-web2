package com.vsuscheduleweb.repositories;

import com.vsuscheduleweb.entity.Group;
import org.hibernate.metamodel.model.convert.spi.JpaAttributeConverter;
import org.springframework.data.jpa.repository.JpaRepository;

public interface GroupRepository extends JpaRepository<Group,String> {
}
