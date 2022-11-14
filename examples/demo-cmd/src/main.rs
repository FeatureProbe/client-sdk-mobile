use std::time::Duration;

use feature_probe_mobile_sdk_core::{FPConfig, FPUser, FeatureProbe, Url};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let toggles_url = Url::parse("https://featureprobe.io/server/api/client-sdk/toggles").unwrap();
    let events_url = Url::parse("https://featureprobe.io/server/api/events").unwrap();
    let realtime_url = Url::parse("https://featureprobe.io/server/realtime").unwrap();
    let client_sdk_key = "client-75d9182a7724b03d531178142b9031b831e464fe".to_owned();
    let refresh_interval = Duration::from_secs(100);
    let start_wait = Some(Duration::from_secs(3));
    let config = FPConfig {
        toggles_url,
        events_url,
        realtime_url,
        client_sdk_key,
        refresh_interval,
        start_wait,
    };

    let user = FPUser::new("uniq_key");

    let fp = FeatureProbe::new(config, user);

    loop {
        let d = fp.bool_detail("campaign_allow_list", false);
        info!("detail {:?}", d);

        tokio::time::sleep(Duration::from_secs(3)).await;
    }
}
