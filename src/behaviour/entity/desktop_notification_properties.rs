use indradb::NamedProperty;
use inexor_rgf_core_reactive::NamedProperties;
use notify_rust::Timeout;
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display, PartialEq, Hash)]
pub enum DesktopNotificationProperties {
    #[strum(serialize = "show")]
    SHOW,
    #[strum(serialize = "app_name")]
    APP_NAME,
    #[strum(serialize = "summary")]
    SUMMARY,
    #[strum(serialize = "body")]
    BODY,
    #[strum(serialize = "icon")]
    ICON,
    #[strum(serialize = "timeout")]
    TIMEOUT,
}

impl DesktopNotificationProperties {
    pub fn default_value(&self) -> Value {
        let default_timeout: i32 = Timeout::Default.into();
        match self {
            DesktopNotificationProperties::SHOW => json!(false),
            DesktopNotificationProperties::APP_NAME => json!(""),
            DesktopNotificationProperties::SUMMARY => json!(""),
            DesktopNotificationProperties::BODY => json!(""),
            DesktopNotificationProperties::ICON => json!(""),
            DesktopNotificationProperties::TIMEOUT => json!(default_timeout),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(DesktopNotificationProperties::SHOW),
            NamedProperty::from(DesktopNotificationProperties::APP_NAME),
            NamedProperty::from(DesktopNotificationProperties::SUMMARY),
            NamedProperty::from(DesktopNotificationProperties::BODY),
            NamedProperty::from(DesktopNotificationProperties::ICON),
            NamedProperty::from(DesktopNotificationProperties::TIMEOUT),
        ]
    }
}

impl From<DesktopNotificationProperties> for NamedProperty {
    fn from(p: DesktopNotificationProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: p.default_value(),
        }
    }
}

impl From<DesktopNotificationProperties> for String {
    fn from(p: DesktopNotificationProperties) -> Self {
        p.to_string()
    }
}

impl Eq for DesktopNotificationProperties {}
