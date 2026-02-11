use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ListLessonResponse {
    #[serde(rename = "lessonResponses")]
    pub lesson_responses: Vec<LessonResponse>,
}

#[derive(Debug, Serialize)]
pub struct LessonResponse {
    pub id: uuid::Uuid,
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    pub auditorium: Option<String>,
    pub date: Option<String>,
    #[serde(rename = "weekDay")]
    pub week_day: Option<String>,
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    #[serde(rename = "teacherId")]
    pub teacher_id: Option<i32>,
    pub teacher: Option<TeacherResponse>,
    #[serde(rename = "subgroupId")]
    pub subgroup_id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub lesson_type: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TeacherResponse {
    pub id: Option<i32>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub surname: Option<String>,
    pub initials: Option<String>,
    #[serde(rename = "imgLink")]
    pub img_link: Option<String>,
    pub description: Option<String>,
    pub fullname: Option<String>,
    pub qualification: Option<String>,
}
