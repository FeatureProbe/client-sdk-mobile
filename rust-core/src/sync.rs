use crate::{FPDetail, Repository};
use headers::HeaderValue;
use http::StatusCode;
use parking_lot::RwLock;
use reqwest::{header::AUTHORIZATION, Client, Method};
use serde_json::Value;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tracing::{debug, warn};
use url::Url;

#[derive(Debug, Clone)]
pub struct Synchronizer {
    inner: Arc<Inner>,
}

#[derive(Debug)]
struct Inner {
    remote_url: Url,
    refresh_interval: Duration,
    auth: HeaderValue,
    client: Option<Client>,
    repo: Arc<RwLock<Repository>>,
}

//TODO: graceful shutdown
impl Synchronizer {
    pub fn new(
        remote_url: Url,
        refresh_interval: Duration,
        auth: HeaderValue,
        repo: Arc<RwLock<Repository>>,
    ) -> Self {
        Self {
            inner: Arc::new(Inner {
                remote_url,
                refresh_interval,
                auth,
                client: None,
                repo,
            }),
        }
    }

    pub fn sync(&self, wait_first_resp: bool) {
        use std::sync::mpsc::sync_channel;
        let inner = self.inner.clone();
        let client = match &self.inner.client {
            Some(c) => c.clone(),
            None => reqwest::Client::new(),
        };
        let (tx, rx) = sync_channel(1);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(inner.refresh_interval);
            if wait_first_resp {
                inner.do_sync(&client).await;
                let _ = tx.send(true);
                interval.tick().await;
            }
            loop {
                inner.do_sync(&client).await;
                interval.tick().await;
            }
        });

        if wait_first_resp {
            let _ = rx.recv();
        }
    }

    #[cfg(test)]
    pub fn repository(&self) -> Arc<RwLock<Repository>> {
        self.inner.repo.clone()
    }
}

impl Inner {
    async fn do_sync(&self, client: &Client) {
        let request = client
            .request(Method::GET, self.remote_url.clone())
            .header(AUTHORIZATION, self.auth.clone())
            .timeout(self.refresh_interval);

        //TODO: report failure
        match request.send().await {
            Err(e) => debug!("sync http error: {}", e),
            Ok(resp) => {
                let status = resp.status();
                match status {
                    StatusCode::OK => match resp.text().await {
                        Err(e) => debug!("sync response error: {}", e),
                        Ok(body) => {
                            match serde_json::from_str::<HashMap<String, FPDetail<Value>>>(&body) {
                                Err(e) => {
                                    debug!("sync json error: {} body: {}", e.to_string(), body)
                                }
                                Ok(r) => {
                                    // TODO: validate repo
                                    // TODO: diff change, notify subscriber
                                    debug!("sync success {:?}", r);
                                    let mut repo = self.repo.write();
                                    *repo = r
                                }
                            }
                        }
                    },
                    _ => warn!("sync http failed: status code {}", status),
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
        repo::SdkRepository,
        ServerConfig,
    };
    use http::{header, StatusCode};
    use std::{fs, net::SocketAddr, path::PathBuf};

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn test_sync() {
        let port = 19009;
        let server_port = 19010;
        setup_mock_api(port).await;
        setup_fp_server(port, server_port, "client-sdk-key", "server-sdk-key").await;
        let syncer = build_synchronizer(server_port);
        syncer.sync(true);

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
                client: None,
                repo: Default::default(),
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
        let repo = SdkRepository::new(ServerConfig {
            toggles_url,
            keys_url: None,
            server_port,
            events_url: events_url.clone(),
            refresh_interval: Duration::from_secs(1),
            client_sdk_key: Some(client_sdk_key.to_owned()),
            server_sdk_key: Some(server_sdk_key.to_owned()),
        });
        repo.sync(client_sdk_key.to_owned(), server_sdk_key.to_owned());
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
