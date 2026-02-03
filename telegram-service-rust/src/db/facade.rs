use log::{debug, error, info};
use sqlx::PgPool;

use crate::db::{students_repo, user_states_repo};
use crate::domain::registration_state::RegistrationState;
use crate::domain::student::Student;
use crate::domain::user_state::UserState;

/// Facade над DB-слоем.
///
/// Зачем он нужен:
/// - скрывает детали репозиториев (SQL/таблицы/структуру хранения)
/// - даёт "use-case" методы для верхнего слоя (бота)
/// - централизует логирование DB операций на уровне бизнес-действий
///
/// Важно:
/// `DbFacade` хранит `PgPool`. Сам `PgPool` внутри `sqlx` является
/// дешёвым для клонирования хэндлом (как Arc), поэтому `DbFacade` тоже Clone.
#[derive(Clone)]
pub struct DbFacade {
    pool: PgPool,
}

impl DbFacade {
    /// Создаём фасад на базе уже инициализированного пула соединений.
    ///
    /// Это "граница инфраструктуры": пул уже должен быть валиден.
    /// Лог тут полезен для диагностики старта, но не должен быть слишком шумным.
    pub fn new(pool: PgPool) -> Self {
        info!("Создан DbFacade");
        Self { pool }
    }

    /// Найти студента по telegram_id.
    ///
    /// Диагностика:
    /// - debug логируем ключ поиска, потому что это частый запрос
    /// - ошибки не логируем здесь насильно: пусть решает вызывающая сторона
    ///   (иногда `Err` — это важный сигнал, иногда его намеренно "глушат").
    pub async fn find_student(&self, telegram_id: i64) -> anyhow::Result<Option<Student>> {
        debug!("DB: find_student telegram_id={}", mask_telegram_id(telegram_id));
        students_repo::find_by_telegram_id(&self.pool, telegram_id).await
    }

    /// Зарегистрировать/сохранить студента.
    ///
    /// Это ключевая точка:
    /// - данные пользователя попадают в "постоянное" хранилище
    /// - возможны конфликты/ограничения в БД (unique, not null и т.д.)
    ///
    /// Логи:
    /// - info: факт попытки регистрации (business event)
    /// - error: если вставка не удалась (diagnostic event)
    pub async fn register_student(
        &self,
        telegram_id: i64,
        faculty: &str,
        group: &str,
        study_form: &str,
        course: Option<&str>,
        username: Option<&str>,
    ) -> anyhow::Result<Student> {
        info!(
            "DB: register_student telegram_id={}, faculty='{}', group='{}', study_form='{}', course={:?}, has_username={}",
            mask_telegram_id(telegram_id),
            faculty,
            group,
            study_form,
            course,
            username.is_some()
        );

        match students_repo::insert(
            &self.pool,
            telegram_id,
            faculty,
            group,
            study_form,
            course,
            username,
        )
            .await
        {
            Ok(student) => {
                debug!(
                    "DB: register_student success telegram_id={}",
                    mask_telegram_id(telegram_id)
                );
                Ok(student)
            }
            Err(err) => {
                // error оправдан: регистрация — важное бизнес-действие,
                // и ошибка почти всегда требует внимания (конфиг БД, схема, ограничения).
                error!(
                    "DB: register_student failed telegram_id={} err={:?}",
                    mask_telegram_id(telegram_id),
                    err
                );
                Err(err)
            }
        }
    }

    /// Получить состояние пользователя (сессия/регистрация/UI-message).
    ///
    /// Это "оперативное" состояние, которое помогает боту помнить,
    /// на каком шаге находится пользователь.
    pub async fn get_user_state(&self, telegram_id: i64) -> anyhow::Result<Option<UserState>> {
        debug!("DB: get_user_state telegram_id={}", mask_telegram_id(telegram_id));
        user_states_repo::find_by_telegram_id(&self.pool, telegram_id).await
    }

    /// Сбросить регистрацию и перевести пользователя в заданное состояние.
    ///
    /// Используется для начала/перезапуска сценария регистрации.
    /// Логируем как debug: это часто вызывается и не всегда "событие уровня info".
    pub async fn reset_registration(
        &self,
        telegram_id: i64,
        state: RegistrationState,
    ) -> anyhow::Result<UserState> {
        debug!(
            "DB: reset_registration telegram_id={} -> state={}",
            mask_telegram_id(telegram_id),
            state.as_str()
        );
        user_states_repo::clear_registration(&self.pool, telegram_id, state.as_str()).await
    }

    /// Установить состояние (state machine шаг).
    ///
    /// Это "переключение состояния" внутри регистрации или сценариев UI.
    pub async fn set_state(
        &self,
        telegram_id: i64,
        state: RegistrationState,
    ) -> anyhow::Result<UserState> {
        debug!(
            "DB: set_state telegram_id={} -> state={}",
            mask_telegram_id(telegram_id),
            state.as_str()
        );
        user_states_repo::upsert_state(&self.pool, telegram_id, state.as_str()).await
    }

    /// Установить факультет пользователя в процессе регистрации.
    ///
    /// Вместе с установкой faculty обычно сразу двигаем state machine дальше,
    /// чтобы бот мог продолжить сценарий с правильного шага.
    pub async fn set_user_faculty(
        &self,
        telegram_id: i64,
        faculty: &str,
        state: RegistrationState,
    ) -> anyhow::Result<UserState> {
        debug!(
            "DB: set_user_faculty telegram_id={} faculty='{}' -> state={}",
            mask_telegram_id(telegram_id),
            faculty,
            state.as_str()
        );
        user_states_repo::update_faculty(&self.pool, telegram_id, faculty, state.as_str()).await
    }

    /// Установить форму обучения пользователя в процессе регистрации.
    pub async fn set_user_study_form(
        &self,
        telegram_id: i64,
        form: &str,
        state: RegistrationState,
    ) -> anyhow::Result<UserState> {
        debug!(
            "DB: set_user_study_form telegram_id={} form='{}' -> state={}",
            mask_telegram_id(telegram_id),
            form,
            state.as_str()
        );
        user_states_repo::update_study_form(&self.pool, telegram_id, form, state.as_str()).await
    }

    /// Установить курс пользователя в процессе регистрации.
    pub async fn set_user_course(
        &self,
        telegram_id: i64,
        course: &str,
        state: RegistrationState,
    ) -> anyhow::Result<UserState> {
        debug!(
            "DB: set_user_course telegram_id={} course='{}' -> state={}",
            mask_telegram_id(telegram_id),
            course,
            state.as_str()
        );
        user_states_repo::update_course(&self.pool, telegram_id, course, state.as_str()).await
    }

    /// Получить сохранённый UI message (чат + id сообщения), который бот пытается редактировать.
    ///
    /// Зачем:
    /// - вместо спама новыми сообщениями бот старается редактировать "последний экран"
    /// - данные хранятся в user_state, поэтому читаем именно его
    ///
    /// Возвращаем Option<(chat_id, message_id)> только если оба значения присутствуют.
    /// Если записан только chat_id или только message_id — считаем UI message неконсистентным
    /// и возвращаем None (fallback на отправку нового сообщения выше по стеку).
    pub async fn get_ui_message(&self, telegram_id: i64) -> anyhow::Result<Option<(i64, i32)>> {
        debug!("DB: get_ui_message telegram_id={}", mask_telegram_id(telegram_id));

        let state = self.get_user_state(telegram_id).await?;

        let ui = state.and_then(|s| {
            s.ui_chat_id
                .and_then(|chat_id| s.ui_message_id.map(|msg_id| (chat_id, msg_id)))
        });

        debug!(
            "DB: get_ui_message result telegram_id={} ui={:?}",
            mask_telegram_id(telegram_id),
            ui
        );

        Ok(ui)
    }

    /// Сохранить "текущее" UI сообщение (чат + id сообщения).
    ///
    /// Важно:
    /// - это вспомогательные данные для UX (редактирование вместо отправки новых сообщений)
    /// - ошибка сохранения UI message обычно не критична для сервиса,
    ///   но мы не глушим её здесь — решение о том, глушить или нет, принимает вызывающий слой.
    pub async fn set_ui_message(
        &self,
        telegram_id: i64,
        chat_id: i64,
        message_id: i32,
    ) -> anyhow::Result<UserState> {
        debug!(
            "DB: set_ui_message telegram_id={} chat_id={} message_id={}",
            mask_telegram_id(telegram_id),
            chat_id,
            message_id
        );
        user_states_repo::update_ui_message(&self.pool, telegram_id, chat_id, message_id).await
    }
}

fn mask_telegram_id(telegram_id: i64) -> String {
    let value = telegram_id.abs().to_string();
    if value.len() <= 4 {
        return "****".to_string();
    }
    let tail = &value[value.len() - 4..];
    format!("***{}", tail)
}
