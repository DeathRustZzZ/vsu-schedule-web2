// @generated automatically by Diesel CLI.

diesel::table! {
    schedule_lessons (id) {
        id -> Int4,
        faculty -> Text,
        group_name -> Text,
        lesson_date -> Date,
        lesson_number -> Int4,
        subject -> Text,
        teacher -> Text,
        room -> Text,
    }
}

diesel::table! {
    students (id) {
        id -> Uuid,
        telegram_id -> Int8,
        faculty -> Text,
        group_name -> Text,
        created_at -> Timestamptz,
        study_form -> Text,
        course -> Text,
        username -> Nullable<Text>,
    }
}

diesel::table! {
    user_states (id) {
        id -> Int4,
        telegram_id -> Int8,
        #[max_length = 50]
        state -> Varchar,
        #[max_length = 100]
        faculty -> Nullable<Varchar>,
        #[max_length = 50]
        study_form -> Nullable<Varchar>,
        #[max_length = 20]
        course -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(schedule_lessons, students, user_states,);
