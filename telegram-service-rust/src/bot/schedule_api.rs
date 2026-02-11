use serde::Deserialize;

/// Клиент для внешнего Schedule API.
///
/// Назначение:
/// - инкапсулировать детали HTTP вызовов (`reqwest`)
/// - дать типобезопасные ответы через `serde`
///
/// Важно:
/// - `reqwest::Client` должен быть один на приложение (или на компонент),
///   потому что внутри он держит пул соединений и DNS кэш.
/// - `base_url` хранится отдельно, чтобы можно было менять окружения (dev/stage/prod).
#[derive(Clone)]
pub struct ScheduleApi {
    base_url: String,
    client: reqwest::Client,
}

impl ScheduleApi {
    /// Создаёт клиент Schedule API.
    ///
    /// Timeout задаётся на уровне клиента, чтобы:
    /// - не зависать вечно при сетевых проблемах
    /// - давать предсказуемое время отклика бота
    ///
    /// `expect` здесь — осознанно: если reqwest client не собрался,
    /// это инфраструктурная ошибка старта приложения (некорректная сборка TLS/конфиг),
    /// и сервис лучше не запускать.
    pub fn new(base_url: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("failed to build http client");

        // Debug-лог полезен, чтобы сразу видеть на какой base_url смотрит бот.
        // (частая причина багов — перепутали dev/prod URL)
        log::info!("ScheduleApi initialized with base_url={}", base_url);

        Self { base_url, client }
    }

    /// Получить список доступных групп по факультету.
    ///
    /// Типовая точка ошибок:
    /// - неверный base_url / неверный путь
    /// - API возвращает 404/500
    /// - API меняет формат JSON → десериализация падает
    pub async fn get_available_groups(
        &self,
        faculty: &str,
    ) -> Result<GroupListResponse, reqwest::Error> {
        // Формируем URL явно — удобно для логов и быстрой диагностики.
        let url = format!("{}/api/v1/groups/available/{}", self.base_url, faculty);

        // Debug: логируем полный URL и входные параметры.
        // В проде это чаще оставляют на debug, чтобы не шуметь.
        log::debug!(
            "ScheduleApi.get_available_groups request: faculty='{}' url='{}'",
            faculty,
            url
        );

        // Вызов в 3 этапа:
        // 1) send() — сетевой запрос (может упасть по сети/таймауту/DNS)
        // 2) error_for_status() — превращает 4xx/5xx в ошибку
        // 3) json() — десериализация тела (может упасть при несовпадении схемы)
        let response = self.client.get(url).send().await?;

        // Полезная диагностика: статус и конечный URL.
        // Если API делает редиректы — response.url() покажет итог.
        log::debug!(
            "ScheduleApi.get_available_groups response: status={} final_url='{}'",
            response.status(),
            response.url()
        );

        // Если статус 4xx/5xx — это НЕ сетевой сбой, а ответ сервера.
        // Такие ошибки важно видеть (хотя бы в warn), иначе бот "молча не работает".
        // Но мы не меняем поведение и не извлекаем body, только логируем статус.
        if response.status().is_client_error() || response.status().is_server_error() {
            log::warn!(
                "ScheduleApi.get_available_groups http error: status={} (faculty='{}')",
                response.status(),
                faculty
            );
        }

        response.error_for_status()?.json().await
    }

    /// Получить расписание по параметрам.
    ///
    /// Здесь особенно часто происходят проблемы из-за некорректных query-параметров
    /// (пустые строки, неправильные значения weekday/subgroup и т.д.)
    pub async fn get_schedule(
        &self,
        faculty: &str,
        group: &str,
        subgroup: &str,
        weekday: &str,
    ) -> Result<ScheduleResponse, reqwest::Error> {
        let url = format!("{}/api/v1/bot/schedule", self.base_url);

        // Debug: логируем все параметры запроса.
        // Это must-have для расследования "почему пользователю показывается не то расписание".
        log::debug!(
            "ScheduleApi.get_schedule request: url='{}' faculty='{}' group='{}' subgroup='{}' weekday='{}'",
            url, faculty, group, subgroup, weekday
        );

        let response = self
            .client
            .get(url)
            .query(&[
                ("faculty", faculty),
                ("group", group),
                ("subgroup", subgroup),
                ("weekday", weekday),
            ])
            .send()
            .await?;

        log::debug!(
            "ScheduleApi.get_schedule response: status={} final_url='{}'",
            response.status(),
            response.url()
        );

        // Предупреждаем о проблемных статусах — помогает отличать
        // "API упало" от "у нас баг в десериализации" и от "параметры неправильные".
        if response.status().is_client_error() || response.status().is_server_error() {
            log::warn!(
                "ScheduleApi.get_schedule http error: status={} (faculty='{}', group='{}', subgroup='{}', weekday='{}')",
                response.status(),
                faculty,
                group,
                subgroup,
                weekday
            );
        }

        response.error_for_status()?.json().await
    }
}

/// Ответ API со списком групп.
///
/// rename’ы фиксируют контракт с внешним сервисом.
/// Это особенно важно: если API меняет названия полей — сразу ломается десериализация,
/// а не начинается "тихая" работа с пустыми данными.
#[derive(Debug, Deserialize)]
pub struct GroupListResponse {
    /// Список групп и доступных подгрупп для каждой.
    #[serde(rename = "listGroupWithSubgroupsIds")]
    pub list: Vec<GroupWithSubgroupsIds>,
}

/// Элемент списка групп: идентификатор группы + доступные подгруппы.
///
/// Clone полезен, если дальше мы хотим кэшировать/переиспользовать эти данные между хендлерами.
#[derive(Debug, Deserialize, Clone)]
pub struct GroupWithSubgroupsIds {
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "subgroupIds")]
    pub subgroup_ids: Vec<String>,
}

/// Ответ API с расписанием: список занятий.
#[derive(Debug, Deserialize)]
pub struct ScheduleResponse {
    #[serde(rename = "lessonResponses")]
    pub lessons: Vec<LessonResponse>,
}

/// Модель урока, как она приходит из внешнего API.
///
/// Важно:
/// - большинство полей здесь String/Option<String>, потому что мы принимаем данные "как есть"
///   и не накладываем доменные ограничения на уровне транспорта.
/// - доменную валидацию (например, корректность времени) лучше делать выше, в domain-слое.
#[derive(Debug, Deserialize)]
pub struct LessonResponse {
    pub id: String,

    #[serde(rename = "startTime")]
    pub start_time: String,

    #[serde(rename = "endTime")]
    pub end_time: String,

    /// Аудитория может отсутствовать (онлайн/уточняется/не задано).
    pub auditorium: Option<String>,

    /// Дата занятия строкой (формат зависит от API).
    /// Если понадобится строгая работа с датой, это обычно конвертят на уровне domain.
    pub date: String,

    #[serde(rename = "weekDay")]
    pub week_day: String,

    #[serde(rename = "groupId")]
    pub group_id: String,

    pub teacher: Option<TeacherResponse>,

    #[serde(rename = "teacherId")]
    pub teacher_id: Option<i32>,

    #[serde(rename = "subgroupId")]
    pub subgroup_id: Option<String>,

    pub name: String,

    /// Название поля `type` конфликтует с ключевым словом Rust, поэтому `lesson_type`.
    #[serde(rename = "type")]
    pub lesson_type: Option<String>,
}

/// Преподаватель, как он приходит из Schedule API.
#[derive(Debug, Deserialize, Clone)]
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
