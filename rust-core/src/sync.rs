use crate::{FPDetail, FPError, Repository};
use headers::HeaderValue;
use http::StatusCode;
use parking_lot::RwLock;
use reqwest::{header::AUTHORIZATION, header::USER_AGENT, Client, Method};
use serde_json::Value;
use std::{
    collections::HashMap,
    sync::{mpsc::sync_channel, Arc},
    time::{Duration, Instant},
};
use tracing::{debug, error, trace};
use url::Url;

#[derive(Debug, Clone)]
pub struct Synchronizer {
    inner: Arc<Inner>,
}

#[derive(Debug)]
pub enum SyncType {
    Realtime,
    Polling,
}

#[derive(Debug)]
struct Inner {
    remote_url: Url,
    refresh_interval: Duration,
    auth: HeaderValue,
    client: Client,
    repo: Arc<RwLock<Repository>>,
    should_stop: Arc<RwLock<bool>>,
}

//TODO: graceful shutdown
impl Synchronizer {
    pub fn new(
        remote_url: Url,
        refresh_interval: Duration,
        auth: HeaderValue,
        repo: Arc<RwLock<Repository>>,
        should_stop: Arc<RwLock<bool>>,
        client: Client,
    ) -> Self {
        Self {
            inner: Arc::new(Inner {
                remote_url,
                refresh_interval,
                auth,
                client,
                repo,
                should_stop,
            }),
        }
    }

    pub fn start_sync(&self, start_wait: Option<Duration>) {
        let should_stop = self.inner.should_stop.clone();
        let inner = self.inner.clone();
        let (tx, rx) = sync_channel(1);
        let start = Instant::now();
        let mut is_send = false;
        let interval_duration = inner.refresh_interval;
        let is_timeout = Self::init_timeout_fn(start_wait, interval_duration, start);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(inner.refresh_interval);
            loop {
                let result = inner.sync_now(SyncType::Polling).await;

                if let Some(r) = Self::should_send(result, &is_timeout, is_send) {
                    is_send = true;
                    let _ = tx.try_send(r);
                }

                if *should_stop.read() {
                    break;
                }
                interval.tick().await;
            }
        });

        if start_wait.is_some() {
            let _ = rx.recv();
        }
    }

    pub async fn sync_now(&self, t: SyncType) -> Result<(), FPError> {
        self.inner.sync_now(t).await
    }

    #[cfg(test)]
    pub fn repository(&self) -> Arc<RwLock<Repository>> {
        self.inner.repo.clone()
    }

    fn init_timeout_fn(
        start_wait: Option<Duration>,
        interval: Duration,
        start: Instant,
    ) -> Option<Box<dyn Fn() -> bool + Send>> {
        match start_wait {
            Some(timeout) => Some(Box::new(move || start.elapsed() + interval > timeout)),
            None => None,
        }
    }

    fn should_send(
        result: Result<(), FPError>,
        is_timeout: &Option<Box<dyn Fn() -> bool + Send>>,
        is_send: bool,
    ) -> Option<Result<(), FPError>> {
        if let Some(is_timeout) = is_timeout {
            match result {
                Ok(_) if !is_send => {
                    return Some(Ok(()));
                }
                Err(e) if !is_send && is_timeout() => {
                    error!("sync error: {}", e);
                    return Some(Err(e));
                }
                Err(e) => error!("sync error: {}", e),
                _ => {}
            }
        }
        None
    }
}

impl Inner {
    pub async fn sync_now(&self, t: SyncType) -> Result<(), FPError> {
        let request = self
            .client
            .request(Method::GET, self.remote_url.clone())
            .header(AUTHORIZATION, self.auth.clone())
            .header(USER_AGENT, &*crate::USER_AGENT)
            .timeout(self.refresh_interval);

        trace!("sync_now {:?} {:?}", self.auth, t);

        //TODO: report failure
        match request.send().await {
            Err(e) => Err(FPError::HttpError(e.to_string())),
            Ok(resp) => {
                let status = resp.status();
                match status {
                    StatusCode::OK => match resp.text().await {
                        Err(e) => Err(FPError::HttpError(e.to_string())),
                        Ok(body) => {
                            debug!("sync body {:?}", body);
                            match serde_json::from_str::<HashMap<String, FPDetail<Value>>>(&body) {
                                Err(e) => Err(FPError::JsonError(e.to_string())),
                                Ok(r) => {
                                    // TODO: validate repo
                                    // TODO: diff change, notify subscriber
                                    debug!("sync success {:?}", r);
                                    let mut repo = self.repo.write();
                                    *repo = r;
                                    Ok(())
                                }
                            }
                        }
                    },
                    _ => Err(FPError::HttpError(format!(
                        "sync http failed: status code {}",
                        status
                    ))),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FPUser, SdkAuthorization};
    use axum::{
        response::{IntoResponse, Response},
        routing::get,
        Router, TypedHeader,
    };

    use feature_probe_server::{
        http::{serve_http, FpHttpHandler},
        realtime::RealtimeSocket,
        repo::SdkRepository,
        ServerConfig,
    };
    use http::{header, StatusCode};
    use std::{fs, net::SocketAddr, path::PathBuf};

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn test_sync() {
        let api_port = 19009;
        let server_port = 19010;
        let realtime_port = 19011;
        setup_mock_api(api_port).await;
        setup_fp_server(
            api_port,
            server_port,
            realtime_port,
            "client-sdk-key",
            "server-sdk-key",
        )
        .await;
        let syncer = build_synchronizer(server_port);
        syncer.start_sync(Some(Duration::from_secs(5)));

        tokio::time::sleep(Duration::from_millis(200)).await;
        let repo = syncer.repository();
        let repo = repo.read();
        assert!(repo.len() > 0)
    }

    fn build_synchronizer(port: u16) -> Synchronizer {
        let user = FPUser::new("123");
        let mut remote_url =
            Url::parse(&format!("http://127.0.0.1:{}/api/client-sdk/toggles", port)).unwrap();
        remote_url.set_query(Some(&format!("user={}", user.as_base64())));
        let refresh_interval = Duration::from_millis(1000);
        let auth = SdkAuthorization("client-sdk-key".to_owned()).encode();
        Synchronizer {
            inner: Arc::new(Inner {
                remote_url,
                refresh_interval,
                auth,
                client: Default::default(),
                repo: Default::default(),
                should_stop: Default::default(),
            }),
        }
    }

    async fn setup_mock_api(port: u16) {
        let app = Router::new().route("/api/server-sdk/toggles", get(server_sdk_toggles));
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        tokio::spawn(async move {
            let _ = axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await;
        });
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    async fn setup_fp_server(
        target_port: u16,
        server_port: u16,
        realtime_port: u16,
        client_sdk_key: &str,
        server_sdk_key: &str,
    ) -> Arc<SdkRepository> {
        let toggles_url = Url::parse(&format!(
            "http://127.0.0.1:{}/api/server-sdk/toggles",
            target_port
        ))
        .unwrap();
        let events_url =
            Url::parse(&format!("http://127.0.0.1:{}/api/events", target_port)).unwrap();
        let repo = SdkRepository::new(
            ServerConfig {
                toggles_url,
                keys_url: None,
                server_port,
                realtime_port,
                events_url: events_url.clone(),
                refresh_interval: Duration::from_secs(1),
                client_sdk_key: Some(client_sdk_key.to_owned()),
                server_sdk_key: Some(server_sdk_key.to_owned()),
            },
            RealtimeSocket::serve(server_port + 100),
        );
        repo.sync(client_sdk_key.to_owned(), server_sdk_key.to_owned(), 1);
        let repo = Arc::new(repo);
        let feature_probe_server = FpHttpHandler {
            repo: repo.clone(),
            events_url,
            events_timeout: Duration::from_secs(1),
            http_client: Default::default(),
        };
        tokio::spawn(serve_http::<FpHttpHandler>(
            server_port,
            feature_probe_server,
        ));
        tokio::time::sleep(Duration::from_millis(100)).await;
        repo
    }

    async fn server_sdk_toggles(
        TypedHeader(SdkAuthorization(_sdk_key)): TypedHeader<SdkAuthorization>,
    ) -> Response {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/fixtures/repo.json");
        let body = fs::read_to_string(path).unwrap();
        (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            body,
        )
            .into_response()
    }
}
