use crate::sync::Synchronizer;
use crate::user::FPUser;
use crate::{FPDetail, Repository, SdkAuthorization};
use feature_probe_event::event::AccessEvent;
use feature_probe_event::recorder::{unix_timestamp, EventRecorder};
use parking_lot::RwLock;
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;
use url::Url;

#[derive(Debug, Clone)]
pub struct FeatureProbe {
    repo: Arc<RwLock<Repository>>,
    syncer: Option<Synchronizer>,
    event_recorder: Option<EventRecorder>,
    config: FPConfig,
    user: FPUser,
}

#[derive(Debug, Clone)]
pub struct FPConfig {
    pub toggles_url: Url,
    pub events_url: Url,
    pub client_sdk_key: String,
    pub refresh_interval: Duration,
    pub wait_first_resp: bool,
}

#[allow(dead_code)]
impl FeatureProbe {
    pub fn new(config: FPConfig, user: FPUser) -> Self {
        let mut slf = Self {
            config,
            user,
            repo: Default::default(),
            syncer: Default::default(),
            event_recorder: Default::default(),
        };

        slf.start();
        slf
    }

    // Just for test and bench
    pub fn new_with(repo: Repository) -> Self {
        Self {
            repo: Arc::new(RwLock::new(repo)),
            event_recorder: Default::default(),
            syncer: Default::default(),
            user: Default::default(),
            config: FPConfig {
                toggles_url: "http://just_for_test.com".parse().unwrap(),
                events_url: "http://just_for_test.com".parse().unwrap(),
                client_sdk_key: Default::default(),
                refresh_interval: Default::default(),
                wait_first_resp: Default::default(),
            },
        }
    }

    pub fn bool_value(&self, toggle: &str, default: bool) -> bool {
        self.generic_value(toggle, default, |v| v.as_bool())
    }

    pub fn string_value(&self, toggle: &str, default: String) -> String {
        self.generic_value(toggle, default, |v| v.as_str().map(|s| s.to_owned()))
    }

    pub fn number_value(&self, toggle: &str, default: f64) -> f64 {
        self.generic_value(toggle, default, |v| v.as_f64())
    }

    pub fn json_value(&self, toggle: &str, default: Value) -> Value {
        self.generic_value(toggle, default, |v| Some(v.to_owned()))
    }

    pub fn bool_detail(&self, toggle: &str, default: bool) -> FPDetail<bool> {
        self.generic_detail(toggle, default, |v| v.as_bool())
    }

    pub fn string_detail(&self, toggle: &str, default: String) -> FPDetail<String> {
        self.generic_detail(toggle, default, |v| v.as_str().map(|x| x.to_owned()))
    }

    pub fn number_detail(&self, toggle: &str, default: f64) -> FPDetail<f64> {
        self.generic_detail(toggle, default, |v| v.as_f64())
    }

    pub fn json_detail(&self, toggle: &str, default: Value) -> FPDetail<Value> {
        self.generic_detail(toggle, default, |v| Some(v.to_owned()))
    }

    fn generic_value<T>(&self, toggle: &str, default: T, transform: fn(&Value) -> Option<T>) -> T {
        let repo = self.repo.read();
        let detail = repo.get(toggle);
        self.record_detail(toggle, detail);
        match detail {
            None => default,
            Some(d) => match transform(&d.value) {
                None => default,
                Some(v) => v,
            },
        }
    }

    fn generic_detail<T: Default>(
        &self,
        toggle: &str,
        default: T,
        transform: fn(&Value) -> Option<T>,
    ) -> FPDetail<T> {
        let repo = self.repo.read();
        let detail = repo.get(toggle);
        self.record_detail(toggle, detail);
        match detail {
            None => FPDetail {
                value: default,
                reason: format!("Toggle {} not found", toggle),
                rule_index: None,
                version: None,
            },
            Some(d) => match transform(&d.value) {
                None => FPDetail {
                    value: default,
                    reason: "Value type mismatch".to_owned(),
                    rule_index: None,
                    version: None,
                },
                Some(v) => FPDetail {
                    value: v,
                    reason: d.reason.clone(),
                    rule_index: d.rule_index,
                    version: d.version,
                },
            },
        }
    }

    fn record_detail(&self, toggle: &str, detail: Option<&FPDetail<Value>>) -> Option<()> {
        let recorder = self.event_recorder.as_ref()?;
        let detail = detail.as_ref()?;
        let value = &detail.value;
        recorder.record_access(AccessEvent {
            time: unix_timestamp(),
            key: toggle.to_owned(),
            value: value.clone(),
            index: detail.rule_index,
            version: detail.version,
            reason: detail.reason.clone(),
        });
        None
    }

    fn start(&mut self) {
        self.sync();
        self.flush_events();
    }

    fn sync(&mut self) {
        let mut remote_url = self.config.toggles_url.clone();
        remote_url.set_query(Some(&format!("user={}", self.user.as_base64())));

        let refresh_interval = self.config.refresh_interval;
        let auth = SdkAuthorization(self.config.client_sdk_key.clone()).encode();
        let repo = self.repo.clone();
        let syncer = Synchronizer::new(remote_url, refresh_interval, auth, repo);

        syncer.sync(self.config.wait_first_resp);
        self.syncer = Some(syncer);
    }

    fn flush_events(&mut self) {
        let events_url = self.config.events_url.clone();
        let flush_interval = self.config.refresh_interval;
        let auth = SdkAuthorization(self.config.client_sdk_key.clone()).encode();
        let event_recorder = EventRecorder::new(events_url, auth, flush_interval, 100);
        self.event_recorder = Some(event_recorder);
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::FeatureProbe;
    use crate::Repository;
    use std::{collections::HashMap, fs, path::PathBuf};

    #[test]
    fn test_bool() {
        let repo = load_json();
        let fp = FeatureProbe::new_with(repo);
        assert_eq!(fp.bool_value("bool_toggle", false), true);

        let detail = fp.bool_detail("bool_toggle", false);
        assert_eq!(detail.value, true);
        assert_eq!(detail.version, Some(1));
        assert_eq!(detail.rule_index, Some(0));
    }

    #[test]
    fn test_number() {
        let repo = load_json();
        let fp = FeatureProbe::new_with(repo);
        assert_eq!(fp.number_value("number_toggle", 0.0), 1.0);

        let detail = fp.number_detail("number_toggle", 0.0);
        assert_eq!(detail.value, 1.0);
        assert_eq!(detail.version, Some(1));
        assert_eq!(detail.rule_index, Some(0));
    }

    #[test]
    fn test_string() {
        let repo = load_json();
        let fp = FeatureProbe::new_with(repo);
        assert_eq!(
            fp.string_value("string_toggle", "0".to_owned()),
            "1".to_owned()
        );

        let detail = fp.string_detail("string_toggle", "0".to_owned());
        assert_eq!(detail.value, "1".to_owned());
        assert_eq!(detail.version, Some(1));
        assert_eq!(detail.rule_index, Some(0));
    }

    #[test]
    fn test_json() {
        let repo = load_json();
        let mut expect = HashMap::new();
        expect.insert("v".to_owned(), "v1".to_owned());
        expect.insert("variation_0".to_owned(), "c2".to_owned());

        let fp = FeatureProbe::new_with(repo);
        assert_eq!(
            fp.json_value("json_toggle", json!("".to_owned())),
            json!(expect)
        );

        let detail = fp.json_detail("json_toggle", json!("0"));
        assert_eq!(detail.value, json!(expect));
        assert_eq!(detail.version, Some(1));
        assert_eq!(detail.rule_index, Some(0));
    }

    fn load_json() -> Repository {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/fixtures/toggles.json");
        let json_str = fs::read_to_string(path).unwrap();
        serde_json::from_str(&json_str).unwrap()
    }
}
