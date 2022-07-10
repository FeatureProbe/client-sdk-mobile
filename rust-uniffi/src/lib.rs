use feature_probe_mobile_sdk_core::FPConfig as CoreFPConfig;
use feature_probe_mobile_sdk_core::FPDetail;
use feature_probe_mobile_sdk_core::FPUser as CoreFPUser;
use feature_probe_mobile_sdk_core::FeatureProbe as CoreFeatureProbe;
use feature_probe_mobile_sdk_core::Url;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use serde::Serialize;
use serde_json::Value;
use std::time::Duration;
use std::{collections::HashMap, sync::Arc};
use tokio::runtime::{Builder, Runtime};

lazy_static! {
    pub static ref TOKIO_RUNTIME: Runtime = Builder::new_multi_thread()
        .enable_all()
        .worker_threads(4)
        .thread_name("featureprobe")
        .build()
        .expect("can not start tokio runtime");
}

struct FeatureProbe {
    core: CoreFeatureProbe,
}

impl FeatureProbe {
    fn new(config: Arc<FPConfig>, user: Arc<FPUser>) -> Self {
        let _enter = TOKIO_RUNTIME.enter();
        let c_user =
            CoreFPUser::new(user.key.clone()).with_attrs(user.attrs.lock().clone().into_iter());

        let c_config = CoreFPConfig {
            toggles_url: config.remote_url.toggles_url.clone(),
            events_url: config.remote_url.events_url.clone(),
            client_sdk_key: config.client_sdk_key.clone(),
            wait_first_resp: config.wait_first_resp,
            refresh_interval: Duration::from_secs(config.refresh_interval as u64),
        };

        let core = CoreFeatureProbe::new(c_config, c_user);
        FeatureProbe { core }
    }

    fn bool_value(&self, toggle: &str, default_value: bool) -> bool {
        self.core.bool_value(toggle, default_value)
    }

    fn bool_detail(&self, toggle: &str, default_value: bool) -> FPBoolDetail {
        let d = self.core.bool_detail(toggle, default_value);
        FPBoolDetail {
            value: d.value,
            rule_index: d.rule_index.map(|f| f as u16),
            version: d.version,
            reason: d.reason,
        }
    }

    fn number_value(&self, toggle: &str, default_value: f64) -> f64 {
        self.core.number_value(toggle, default_value)
    }

    fn number_detail(&self, toggle: &str, default_value: f64) -> FPNumDetail {
        let d = self.core.number_detail(toggle, default_value);
        FPNumDetail {
            value: d.value,
            rule_index: d.rule_index.map(|f| f as u16),
            version: d.version,
            reason: d.reason,
        }
    }

    fn string_value(&self, toggle: &str, default_value: String) -> String {
        self.core.string_value(toggle, default_value)
    }

    fn string_detail(&self, toggle: &str, default_value: String) -> FPStrDetail {
        let d = self.core.string_detail(toggle, default_value);
        FPStrDetail {
            value: d.value,
            rule_index: d.rule_index.map(|f| f as u16),
            version: d.version,
            reason: d.reason,
        }
    }

    fn json_value(&self, toggle: &str, default_value: String) -> String {
        let default_value = serde_json::from_str(&default_value).expect("invalid default_value");
        let v = self.core.json_value(toggle, default_value);
        serde_json::to_string(&v).expect("invalid json")
    }

    fn json_detail(&self, toggle: &str, default_value: String) -> FPJsonDetail {
        let default_value = serde_json::from_str(&default_value).expect("invalid default_value");
        let d = self.core.json_detail(toggle, default_value);
        let value = serde_json::to_string(&d.value).expect("invalid json");
        FPJsonDetail {
            value,
            rule_index: d.rule_index.map(|f| f as u16),
            version: d.version,
            reason: d.reason,
        }
    }

    fn new_for_test(toggles: String) -> Self {
        let m: HashMap<String, Value> =
            serde_json::from_str(&toggles).expect("invalid default toggles json");

        let repo: HashMap<String, FPDetail<Value>> = m
            .into_iter()
            .map(|(k, value)| {
                (
                    k,
                    FPDetail::<Value> {
                        value,
                        ..Default::default()
                    },
                )
            })
            .collect();

        let core = CoreFeatureProbe::new_with(repo);
        FeatureProbe { core }
    }
}

#[derive(Debug, Default)]
pub struct FPBoolDetail {
    pub value: bool,
    pub rule_index: Option<u16>,
    pub version: Option<u64>,
    pub reason: String,
}

#[derive(Debug, Default)]
pub struct FPNumDetail {
    pub value: f64,
    pub rule_index: Option<u16>,
    pub version: Option<u64>,
    pub reason: String,
}

#[derive(Debug, Default)]
pub struct FPStrDetail {
    pub value: String,
    pub rule_index: Option<u16>,
    pub version: Option<u64>,
    pub reason: String,
}

#[derive(Debug, Default)]
pub struct FPJsonDetail {
    pub value: String,
    pub rule_index: Option<u16>,
    pub version: Option<u64>,
    pub reason: String,
}

#[derive(Debug)]
pub struct FPUrlBuilder {
    remote_url: String,
    toggles_url: Option<String>,
    events_url: Option<String>,
}

impl FPUrlBuilder {
    pub fn new(remote_url: String) -> Self {
        Self {
            remote_url,
            toggles_url: None,
            events_url: None,
        }
    }

    pub fn new_urls(
        remote_url: String,
        toggles_url: Option<String>,
        events_url: Option<String>,
    ) -> Self {
        Self {
            remote_url,
            toggles_url,
            events_url,
        }
    }

    pub fn build(&self) -> Option<Arc<FPUrl>> {
        let remote_url = if !self.remote_url.ends_with('/') {
            format!("{}/", self.remote_url)
        } else {
            self.remote_url.clone()
        };

        let toggles_url = match self.toggles_url {
            None => format!("{}api/client-sdk/toggles", remote_url),
            Some(ref url) => url.clone(),
        };

        let events_url = match self.events_url {
            None => format!("{}api/events", remote_url),
            Some(ref url) => url.clone(),
        };

        let toggles_url = Url::parse(&toggles_url).ok()?;
        let events_url = Url::parse(&events_url).ok()?;
        Some(Arc::new(FPUrl {
            toggles_url,
            events_url,
        }))
    }
}

#[derive(Debug)]
pub struct FPUrl {
    pub toggles_url: Url,
    pub events_url: Url,
}

#[derive(Debug)]
pub struct FPConfig {
    pub remote_url: Arc<FPUrl>,
    pub client_sdk_key: String,
    pub refresh_interval: u8,
    pub wait_first_resp: bool,
}

impl FPConfig {
    fn new(
        remote_url: Arc<FPUrl>,
        client_sdk_key: String,
        refresh_interval: u8,
        wait_first_resp: bool,
    ) -> Self {
        FPConfig {
            remote_url,
            client_sdk_key,
            refresh_interval,
            wait_first_resp,
        }
    }
}

#[derive(Default, Serialize, Debug)]
struct FPUser {
    pub key: String,
    pub attrs: Mutex<HashMap<String, String>>,
}

impl FPUser {
    fn new(user_key: String) -> Self {
        Self {
            key: user_key,
            attrs: Default::default(),
        }
    }

    fn set_attr(&self, key: String, value: String) {
        let mut attrs = self.attrs.lock();
        attrs.insert(key, value);
    }
}

uniffi_macros::include_scaffolding!("featureprobe");
