use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::reactive::BehaviourCreationError;
use log::{error, trace};
use serde_json::Value;

use crate::behaviour::entity::DesktopNotificationProperties;
use crate::model::PropertyInstanceGetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;

use notify_rust::{Notification, Timeout};

pub const DESKTOP_NOTIFICATION: &'static str = "desktop_notification";

pub struct DesktopNotification {
    pub entity: Arc<ReactiveEntityInstance>,

    pub notification: Arc<RwLock<Notification>>,

    pub property_handles: HashMap<DesktopNotificationProperties, u128>,
}

impl DesktopNotification {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<DesktopNotification, BehaviourCreationError> {
        let show = e.properties.get(DesktopNotificationProperties::SHOW.as_ref());
        if show.is_none() {
            error!("Missing property {}", DesktopNotificationProperties::SHOW.as_ref());
            return Err(BehaviourCreationError.into());
        }
        let show = show.unwrap().as_bool().unwrap_or(false);

        let notification = Notification::new()
            .appname(get_value_or_default(e.clone(), DesktopNotificationProperties::APP_NAME).as_str().unwrap())
            .summary(get_value_or_default(e.clone(), DesktopNotificationProperties::SUMMARY).as_str().unwrap())
            .body(get_value_or_default(e.clone(), DesktopNotificationProperties::BODY).as_str().unwrap())
            .icon(get_value_or_default(e.clone(), DesktopNotificationProperties::ICON).as_str().unwrap())
            .timeout(get_value_or_default(e.clone(), DesktopNotificationProperties::TIMEOUT).as_i64().unwrap() as i32)
            .finalize();
        let notification = Arc::new(RwLock::new(notification));

        if show {
            let _result = notification.read().unwrap().show();
        }

        let mut property_handles = HashMap::new();

        let handle_id = handle_property(DesktopNotificationProperties::SHOW, e.clone(), notification.clone(), move |v, notification| {
            if !v.is_boolean() {
                return;
            }
            let show = v.as_bool().unwrap();
            if show {
                let _result = notification.read().unwrap().show();
            }
        });
        property_handles.insert(DesktopNotificationProperties::SHOW, handle_id);

        property_handles.insert(
            DesktopNotificationProperties::SUMMARY,
            handle_property(DesktopNotificationProperties::SUMMARY, e.clone(), notification.clone(), move |v, notification| {
                if !v.is_string() {
                    return;
                }
                notification.write().unwrap().summary(v.as_str().unwrap());
            }),
        );

        property_handles.insert(
            DesktopNotificationProperties::BODY,
            handle_property(DesktopNotificationProperties::BODY, e.clone(), notification.clone(), move |v, notification| {
                if !v.is_string() {
                    return;
                }
                notification.write().unwrap().body(v.as_str().unwrap());
            }),
        );

        property_handles.insert(
            DesktopNotificationProperties::ICON,
            handle_property(DesktopNotificationProperties::ICON, e.clone(), notification.clone(), move |v, notification| {
                if !v.is_string() {
                    return;
                }
                notification.write().unwrap().icon(v.as_str().unwrap());
            }),
        );

        property_handles.insert(
            DesktopNotificationProperties::TIMEOUT,
            handle_property(DesktopNotificationProperties::TIMEOUT, e.clone(), notification.clone(), move |v, notification| {
                if !v.is_number() {
                    return;
                }
                notification.write().unwrap().timeout(Timeout::from(v.as_i64().unwrap() as i32));
            }),
        );

        Ok(DesktopNotification {
            entity: e.clone(),
            notification,
            property_handles,
        })
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }

    pub fn disconnect_property(&self, property_name: &str, handle_id: u128) {
        let property = self.entity.properties.get(property_name);
        if property.is_some() {
            property.unwrap().stream.read().unwrap().remove(handle_id);
        }
    }
}

impl Disconnectable for DesktopNotification {
    fn disconnect(&self) {
        trace!("Disconnecting {} with id {}", DESKTOP_NOTIFICATION, self.entity.id);
        for (property_name, handle_id) in self.property_handles.iter() {
            self.disconnect_property(property_name.as_ref(), *handle_id);
        }
    }
}

/// Automatically disconnect streams on destruction
impl Drop for DesktopNotification {
    fn drop(&mut self) {
        self.disconnect();
    }
}

fn get_value_or_default(entity: Arc<ReactiveEntityInstance>, property: DesktopNotificationProperties) -> Value {
    entity.get(property.as_ref()).unwrap_or(property.default_value())
}

fn handle_property(
    property: DesktopNotificationProperties,
    entity: Arc<ReactiveEntityInstance>,
    notification: Arc<RwLock<Notification>>,
    f: fn(&Value, &Arc<RwLock<Notification>>),
) -> u128 {
    // , f: FnMut(&Value, &Arc<RwLock<Notification>>)
    let handle_id = entity.properties.get(property.as_ref()).unwrap().id.as_u128();
    // let f: dyn FnMut(&Value, &Arc<RwLock<Notification>>) = x;
    entity.properties.get(property.as_ref()).unwrap().stream.read().unwrap().observe_with_handle(
        move |v: &Value| {
            f(v, &notification);
        },
        handle_id,
    );
    handle_id
}
