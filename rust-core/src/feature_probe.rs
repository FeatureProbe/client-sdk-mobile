use crate::sync::{SyncType, Synchronizer};
use crate::user::FPUser;
use crate::{FPDetail, Repository, SdkAuthorization};
use feature_probe_event::event::{AccessEvent, CustomEvent, DebugEvent, Event};
use feature_probe_event::recorder::{unix_timestamp, EventRecorder};
use futures_util::FutureExt;
use parking_lot::RwLock;
use serde_json::Value;
use socketio_rs::Client;
use std::sync::Arc;
use std::time::Duration;
use tracing::trace;
use url::Url;

type SocketCallback = std::pin::Pin<Box<dyn futures_util::Future<Output = ()> + Send>>;

#[derive(Clone)]
pub struct FeatureProbe {
    repo: Arc<RwLock<Repository>>,
    syncer: Option<Synchronizer>,
    event_recorder: Option<EventRecorder>,
    config: FPConfig,
    user: FPUser,
    should_stop: Arc<RwLock<bool>>,
    socket: Option<Client>,
}

#[derive(Debug, Clone)]
pub struct FPConfig {
    pub toggles_url: Url,
    pub events_url: Url,
    pub realtime_url: Url,
    pub client_sdk_key: String,
    pub refresh_interval: Duration,
    pub start_wait: Option<Duration>,
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
            should_stop: Arc::new(RwLock::new(false)),
            socket: None,
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
            should_stop: Arc::new(RwLock::new(false)),
            socket: None,
            config: FPConfig {
                toggles_url: "https://just_for_test.com".parse().unwrap(),
                events_url: "https://just_for_test.com".parse().unwrap(),
                realtime_url: "https://just_for_test.com".parse().unwrap(),
                client_sdk_key: Default::default(),
                refresh_interval: Default::default(),
                start_wait: Default::default(),
            },
        }
    }

    pub fn close(&self) {
        // TODO: logging
        if let Some(recorder) = &self.event_recorder {
            recorder.flush();
        }
        let mut should_stop = self.should_stop.write();
        *should_stop = true;
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

    pub fn track_event(&self, name: &str, value: Option<f64>) {
        if let Some(r) = &self.event_recorder {
            r.record_event(Event::CustomEvent(CustomEvent {
                kind: "custom".to_string(),
                time: unix_timestamp(),
                user: self.user.key.clone(),
                name: name.to_string(),
                value,
            }))
        }
    }

    fn generic_value<T>(&self, toggle: &str, default: T, transform: fn(&Value) -> Option<T>) -> T {
        let repo = self.repo.read();
        let detail = repo.get(toggle);

        detail.map(|d| self.record_event(toggle, d.clone()));

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

        detail.map(|d| self.record_event(toggle, d.clone()));

        match detail {
            None => FPDetail {
                value: default,
                reason: format!("Toggle {} not found", toggle),
                ..Default::default()
            },
            Some(d) => match transform(&d.value) {
                None => FPDetail {
                    value: default,
                    reason: "Value type mismatch".to_owned(),
                    ..Default::default()
                },
                Some(v) => FPDetail {
                    value: v,
                    reason: d.reason.clone(),
                    rule_index: d.rule_index,
                    variation_index: d.variation_index,
                    version: d.version,
                    track_access_events: d.track_access_events,
                    debug_until_time: d.debug_until_time,
                },
            },
        }
    }

    fn record_event(&self, toggle: &str, detail: FPDetail<Value>) -> Option<()> {
        let recorder = self.event_recorder.clone()?;
        let toggle = toggle.to_owned();
        let user = self.user.clone();
        tokio::spawn(async move {
            let ts = unix_timestamp();
            record_access(&recorder, &user, toggle.clone(), &detail, ts);
            record_debug(
                &recorder,
                &user,
                toggle,
                &detail,
                detail.debug_until_time,
                ts,
            );
        });
        None
    }

    fn start(&mut self) {
        self.sync();
        self.connect_socket();
        self.flush_events();
    }

    fn sync(&mut self) {
        let mut remote_url = self.config.toggles_url.clone();
        remote_url.set_query(Some(&format!("user={}", self.user.as_base64())));

        let refresh_interval = self.config.refresh_interval;
        let auth = SdkAuthorization(self.config.client_sdk_key.clone()).encode();
        let repo = self.repo.clone();
        let should_stop = self.should_stop.clone();
        let client = reqwest::Client::default();
        let syncer = Synchronizer::new(
            remote_url,
            refresh_interval,
            auth,
            repo,
            should_stop,
            client,
        );

        syncer.start_sync(self.config.start_wait);
        self.syncer = Some(syncer);
    }

    fn connect_socket(&mut self) {
        let mut slf = self.clone();
        let slf2 = self.clone();
        tokio::spawn(async move {
            let url = slf.config.realtime_url;
            let nsp = url.path();
            let server_sdk_key = slf.config.client_sdk_key.clone();
            trace!("connect_socket {}", url);
            let client = socketio_rs::ClientBuilder::new(url.clone())
                .namespace(nsp)
                .on(socketio_rs::Event::Connect, move |_, socket, _| {
                    Self::socket_on_connect(socket, server_sdk_key.clone())
                })
                .on(
                    "update",
                    move |payload: Option<socketio_rs::Payload>, _, _| {
                        Self::socket_on_update(slf2.clone(), payload)
                    },
                )
                .on("error", |err, _, _| {
                    async move { tracing::error!("socket on error: {:#?}", err) }.boxed()
                })
                .connect()
                .await;
            match client {
                Err(e) => tracing::error!("connect_socket error: {:?}", e),
                Ok(client) => slf.socket = Some(client),
            };
        });
    }

    fn socket_on_connect(socket: socketio_rs::Socket, server_sdk_key: String) -> SocketCallback {
        let sdk_key = server_sdk_key;
        trace!("socket_on_connect: {:?}", sdk_key);
        async move {
            if let Err(e) = socket
                .emit("register", serde_json::json!({ "key": sdk_key }))
                .await
            {
                tracing::error!("register error: {:?}", e);
            }
        }
        .boxed()
    }

    fn socket_on_update(slf: Self, payload: Option<socketio_rs::Payload>) -> SocketCallback {
        trace!("socket_on_update: {:?}", payload);

        async move {
            if let Some(syncer) = &slf.syncer {
                let _ = syncer.sync_now(SyncType::Realtime).await;
            } else {
                tracing::warn!("socket receive update event, but no synchronizer");
            }
        }
        .boxed()
    }

    fn flush_events(&mut self) {
        let events_url = self.config.events_url.clone();
        let flush_interval = self.config.refresh_interval;
        let auth = SdkAuthorization(self.config.client_sdk_key.clone()).encode();
        let should_stop = self.should_stop.clone();
        let event_recorder = EventRecorder::new(
            events_url,
            auth,
            (*crate::USER_AGENT).clone(),
            flush_interval,
            100,
            should_stop,
        );

        self.event_recorder = Some(event_recorder);
    }
}

fn record_access(
    recorder: &EventRecorder,
    user: &FPUser,
    toggle: String,
    detail: &FPDetail<Value>,
    ts: u128,
) -> Option<()> {
    let value = &detail.value;
    let user = user.key.clone();
    recorder.record_event(Event::AccessEvent(AccessEvent {
        kind: "access".to_string(),
        time: ts,
        key: toggle,
        user,
        value: value.clone(),
        variation_index: detail.variation_index.unwrap_or(0),
        rule_index: detail.rule_index,
        version: detail.version,
        track_access_events: detail.track_access_events,
    }));
    None
}

fn record_debug(
    recorder: &EventRecorder,
    user: &FPUser,
    toggle: String,
    detail: &FPDetail<Value>,
    debug_until_time: Option<u128>,
    ts: u128,
) -> Option<()> {
    let debug_until_time = debug_until_time?;
    let user_detail = serde_json::to_value(user.clone()).ok()?;
    let value = detail.value.clone();
    if debug_until_time >= ts {
        let debug = DebugEvent {
            kind: "debug".to_string(),
            time: ts,
            key: toggle,
            user: user.key.clone(),
            user_detail,
            value,
            variation_index: detail.variation_index?,
            version: detail.version,
            rule_index: detail.rule_index,
            reason: Some(detail.reason.to_string()),
        };
        recorder.record_event(Event::DebugEvent(debug));
    }
    None
}

impl std::fmt::Debug for FeatureProbe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("FeatureProbe")
            .field(&self.repo)
            .field(&self.syncer)
            .field(&self.config)
            .field(&self.should_stop)
            .finish()
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
        assert!(fp.bool_value("bool_toggle", false));

        let detail = fp.bool_detail("bool_toggle", false);
        assert!(detail.value);
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
