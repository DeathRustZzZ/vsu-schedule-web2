//src/services/telegram/menu_state_manager.rs
//! Менеджер состояния меню для отслеживания, где находится пользователь в меню

use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;
use crate::domain::menu::MenuState;

/// Менеджер состояния меню пользователей
pub struct MenuStateManager {
    /// Карта user_id -> текущее состояние меню
    states: Arc<RwLock<HashMap<u64, MenuState>>>,
}

impl MenuStateManager {
    /// Создать новый менеджер состояния
    pub fn new() -> Self {
        Self {
            states: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Получить текущее состояние пользователя
    pub async fn get_state(&self, user_id: u64) -> MenuState {
        let states = self.states.read().await;
        states.get(&user_id).copied().unwrap_or(MenuState::Main)
    }

    /// Установить новое состояние пользователя
    pub async fn set_state(&self, user_id: u64, state: MenuState) {
        let mut states = self.states.write().await;
        states.insert(user_id, state);
    }

    /// Вернуться в главное меню
    pub async fn reset_to_main(&self, user_id: u64) {
        self.set_state(user_id, MenuState::Main).await;
    }

    /// Очистить состояние пользователя
    pub async fn clear_state(&self, user_id: u64) {
        let mut states = self.states.write().await;
        states.remove(&user_id);
    }

    /// Получить размер кэша (для отладки)
    pub async fn cache_size(&self) -> usize {
        self.states.read().await.len()
    }
}

impl Default for MenuStateManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for MenuStateManager {
    fn clone(&self) -> Self {
        Self {
            states: Arc::clone(&self.states),
        }
    }
}

