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
    // pub show_handle_id: u128,
    // pub summary_handle_id: u128,
    // pub body_handle_id: u128,
    // pub icon_handle_id: u128,
    // pub timeout_handle_id: u128,
}

impl DesktopNotification {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<DesktopNotification, BehaviourCreationError> {
        let show = e.properties.get(DesktopNotificationProperties::SHOW.as_ref());
        if show.is_none() {
            error!("Missing property {}", DesktopNotificationProperties::SHOW.as_ref());
            return Err(BehaviourCreationError.into());
        }
        let show = show.unwrap().as_bool().unwrap_or(false);
        // Initial values
        // let app_name = ;
        // let summary = ;
        // let body = get_value_or_default(e.clone(), DesktopNotificationProperties::BODY).as_str().unwrap();
        // let icon = get_value_or_default(e.clone(), DesktopNotificationProperties::ICON).as_str().unwrap();
        // let timeout = get_value_or_default(e.clone(), DesktopNotificationProperties::TIMEOUT).as_i64().unwrap();

        let notification = Notification::new()
            .appname(get_value_or_default(e.clone(), DesktopNotificationProperties::APP_NAME).as_str().unwrap())
            .summary(get_value_or_default(e.clone(), DesktopNotificationProperties::SUMMARY).as_str().unwrap())
            .body(get_value_or_default(e.clone(), DesktopNotificationProperties::BODY).as_str().unwrap())
            .icon(get_value_or_default(e.clone(), DesktopNotificationProperties::ICON).as_str().unwrap())
            .timeout(get_value_or_default(e.clone(), DesktopNotificationProperties::TIMEOUT).as_i64().unwrap() as i32)
            .finalize();
        let notification = Arc::new(RwLock::new(notification));

        if show {
            notification.read().unwrap().show();
        }

        let mut property_handles = HashMap::new();

        // let show_notification = notification.clone();
        // let show_handle_id = e.properties.get(DesktopNotificationProperties::SHOW.as_ref()).unwrap().id.as_u128();
        // e.properties
        //     .get(DesktopNotificationProperties::SHOW.as_ref())
        //     .unwrap()
        //     .stream
        //     .read()
        //     .unwrap()
        //     .observe_with_handle(
        //         move |v: &Value| {
        //             if !v.is_boolean() {
        //                 return;
        //             }
        //             let show = v.as_bool().unwrap();
        //             if show {
        //                 show_notification.read().unwrap().show();
        //             }
        //         },
        //         show_handle_id,
        //     );
        let handle_id = handle_property(DesktopNotificationProperties::SHOW, e.clone(), notification.clone(), move |v, notification| {
            if !v.is_boolean() {
                return;
            }
            let show = v.as_bool().unwrap();
            if show {
                notification.read().unwrap().show();
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

        // let summary_notification = notification.clone();
        // let summary_handle_id = e.properties.get(DesktopNotificationProperties::SUMMARY.as_ref()).unwrap().id.as_u128();
        // e.properties
        //     .get(DesktopNotificationProperties::SUMMARY.as_ref())
        //     .unwrap()
        //     .stream
        //     .read()
        //     .unwrap()
        //     .observe_with_handle(
        //         move |v: &Value| {
        //             if !v.is_string() {
        //                 return;
        //             }
        //             summary_notification.write().unwrap().icon(v.as_str().unwrap());
        //         },
        //         summary_handle_id,
        //     );
        //
        // let body_notification = notification.clone();
        // let body_handle_id = e.properties.get(DesktopNotificationProperties::BODY.as_ref()).unwrap().id.as_u128();
        // e.properties
        //     .get(DesktopNotificationProperties::BODY.as_ref())
        //     .unwrap()
        //     .stream
        //     .read()
        //     .unwrap()
        //     .observe_with_handle(
        //         move |v: &Value| {
        //             if !v.is_string() {
        //                 return;
        //             }
        //             body_notification.write().unwrap().body(v.as_str().unwrap());
        //         },
        //         body_handle_id,
        //     );
        //
        // let icon_notification = notification.clone();
        // let icon_handle_id = e.properties.get(DesktopNotificationProperties::ICON.as_ref()).unwrap().id.as_u128();
        // e.properties
        //     .get(DesktopNotificationProperties::ICON.as_ref())
        //     .unwrap()
        //     .stream
        //     .read()
        //     .unwrap()
        //     .observe_with_handle(
        //         move |v: &Value| {
        //             if !v.is_string() {
        //                 return;
        //             }
        //             icon_notification.write().unwrap().icon(v.as_str().unwrap());
        //         },
        //         icon_handle_id,
        //     );
        //
        // let timeout_notification = notification.clone();
        // let timeout_handle_id = e.properties.get(DesktopNotificationProperties::TIMEOUT.as_ref()).unwrap().id.as_u128();
        // e.properties
        //     .get(DesktopNotificationProperties::TIMEOUT.as_ref())
        //     .unwrap()
        //     .stream
        //     .read()
        //     .unwrap()
        //     .observe_with_handle(
        //         move |v: &Value| {
        //             if !v.is_string() {
        //                 return;
        //             }
        //             timeout_notification.write().unwrap().icon(v.as_str().unwrap());
        //         },
        //         timeout_handle_id,
        //     );

        Ok(DesktopNotification {
            entity: e.clone(),
            notification,
            property_handles
            // show_handle_id,
            // summary_handle_id,
            // body_handle_id,
            // icon_handle_id,
            // timeout_handle_id,
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
        // self.disconnect_property(DesktopNotificationProperties::SHOW.as_ref(), self.show_handle_id);
        // self.disconnect_property(DesktopNotificationProperties::SUMMARY.as_ref(), self.summary_handle_id);
        // self.disconnect_property(DesktopNotificationProperties::BODY.as_ref(), self.body_handle_id);
        // self.disconnect_property(DesktopNotificationProperties::ICON.as_ref(), self.icon_handle_id);
        // self.disconnect_property(DesktopNotificationProperties::TIMEOUT.as_ref(), self.timeout_handle_id);
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
