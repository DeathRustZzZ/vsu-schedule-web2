-- Увеличиваем размер поля initials для поддержки длинных инициалов (например, "А. Б. В.")
ALTER TABLE "teachers"
    ALTER COLUMN "initials" TYPE varchar(20);

-- Увеличиваем размер поля auditorium для поддержки длинных номеров аудиторий
ALTER TABLE "lessons"
    ALTER COLUMN "auditorium" TYPE varchar(30);
