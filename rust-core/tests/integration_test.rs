use axum::{
    response::{IntoResponse, Response},
    routing::get,
    Router, TypedHeader,
};

use feature_probe_mobile_sdk_core::{FPConfig, FPUser, FeatureProbe, SdkAuthorization, Url};
use feature_probe_server::{
    http::{serve_http, FpHttpHandler},
    repo::SdkRepository,
    ServerConfig,
};
use http::{header, StatusCode};
use std::{fs, net::SocketAddr, path::PathBuf, sync::Arc, time::Duration};

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn integration_test() {
    let port = 19011;
    let server_port = 19012;
    setup_mock_api(port).await;
    setup_fp_server(port, server_port, "client-sdk-key", "server-sdk-key").await;

    let toggles_url = format!("http://127.0.0.1:{}/api/client-sdk/toggles", server_port)
        .parse()
        .unwrap();
    let events_url = format!("http://127.0.0.1:{}/api/events", server_port)
        .parse()
        .unwrap();
    let user = FPUser::new("some-user-key");
    let fp = FeatureProbe::new(
        FPConfig {
            toggles_url,
            events_url,
            client_sdk_key: "client-sdk-key".to_owned(),
            refresh_interval: Duration::from_millis(100),
            wait_first_resp: true,
        },
        user,
    );

    assert_eq!(fp.bool_value("bool_toggle", false), true);

    let detail = fp.bool_detail("bool_toggle", false);
    assert_eq!(detail.value, true);
    assert_eq!(detail.version, Some(1));
    assert_eq!(detail.rule_index, None);
    let reason = detail.reason;
    assert!(reason.contains("default"));
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

    let events_url = Url::parse(&format!("http://127.0.0.1:{}/api/events", target_port)).unwrap();
    let repo = SdkRepository::new(ServerConfig {
        toggles_url,
        server_port,
        keys_url: None,
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
