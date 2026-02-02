-- Your SQL goes here
CREATE TABLE schedule_lessons (
                                  id SERIAL PRIMARY KEY,
                                  faculty TEXT NOT NULL,
                                  group_name TEXT NOT NULL,
                                  lesson_date DATE NOT NULL,
                                  lesson_number INT NOT NULL,
                                  subject TEXT NOT NULL,
                                  teacher TEXT NOT NULL,
                                  room TEXT NOT NULL,
                                  UNIQUE (faculty, group_name, lesson_date, lesson_number)
);