use std::sync::Arc;

use crate::di::*;
use async_trait::async_trait;
use log::debug;
use uuid::Uuid;

use crate::behaviour::entity::desktop_notification::DesktopNotification;
use crate::behaviour::entity::desktop_notification::DESKTOP_NOTIFICATION;
use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityBehaviourProvider;

#[wrapper]
pub struct DesktopNotificationStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<DesktopNotification>>>);

#[provides]
fn create_desktop_notification_storage() -> DesktopNotificationStorage {
    DesktopNotificationStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[async_trait]
pub trait NotificationEntityBehaviourProvider: EntityBehaviourProvider + Send + Sync {
    fn create_desktop_notification(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_desktop_notification(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

pub struct NotificationEntityBehaviourProviderImpl {
    desktop_notification: DesktopNotificationStorage,
}

interfaces!(NotificationEntityBehaviourProviderImpl: dyn EntityBehaviourProvider);

#[component]
impl NotificationEntityBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            desktop_notification: create_desktop_notification_storage(),
        }
    }
}

#[async_trait]
#[provides]
impl NotificationEntityBehaviourProvider for NotificationEntityBehaviourProviderImpl {
    fn create_desktop_notification(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        let device_key = DesktopNotification::new(entity_instance.clone());
        if device_key.is_ok() {
            let desktop_notification = Arc::new(device_key.unwrap());
            self.desktop_notification.0.write().unwrap().insert(id, desktop_notification);
            entity_instance.add_behaviour(DESKTOP_NOTIFICATION);
            debug!("Added behaviour {} to entity instance {}", DESKTOP_NOTIFICATION, id);
        }
    }

    fn remove_desktop_notification(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.desktop_notification.0.write().unwrap().remove(&entity_instance.id);
        entity_instance.remove_behaviour(DESKTOP_NOTIFICATION);
        debug!("Removed behaviour {} from entity instance {}", DESKTOP_NOTIFICATION, entity_instance.id);
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.desktop_notification.0.write().unwrap().contains_key(&id) {
            self.desktop_notification.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", DESKTOP_NOTIFICATION, id);
        }
    }
}

impl EntityBehaviourProvider for NotificationEntityBehaviourProviderImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        match entity_instance.clone().type_name.as_str() {
            DESKTOP_NOTIFICATION => self.create_desktop_notification(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        match entity_instance.clone().type_name.as_str() {
            DESKTOP_NOTIFICATION => self.remove_desktop_notification(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
