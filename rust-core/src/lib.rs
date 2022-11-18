mod feature_probe;
mod sync;
mod user;

pub use crate::user::FPUser;
pub use feature_probe::{FPConfig, FeatureProbe};
use lazy_static::lazy_static;
pub use url::Url;

use headers::{Error, Header, HeaderName, HeaderValue};
use http::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, env};
use thiserror::Error;

include!(concat!(env!("OUT_DIR"), "/target_os.rs"));

pub type Repository = HashMap<String, FPDetail<Value>>;

const VERSION: &str = env!("CARGO_PKG_VERSION");

lazy_static! {
    pub static ref USER_AGENT: String = user_agent();
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FPDetail<T: Default> {
    pub value: T,
    pub rule_index: Option<usize>,
    pub variation_index: Option<usize>,
    pub version: Option<u64>,
    pub reason: String,
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum FPError {
    #[error("invalid json: {0}")]
    JsonError(String),
    #[error("invalid http: {0}")]
    HttpError(String),
    #[error("invalid url: {0}")]
    UrlError(String),
}

#[derive(Debug, Deserialize)]
pub struct SdkAuthorization(pub String);

impl SdkAuthorization {
    pub fn encode(&self) -> HeaderValue {
        HeaderValue::from_str(&self.0).expect("valid header value")
    }
}

impl Header for SdkAuthorization {
    fn name() -> &'static HeaderName {
        &AUTHORIZATION
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        match values.next() {
            Some(v) => match v.to_str() {
                Ok(s) => Ok(SdkAuthorization(s.to_owned())),
                Err(_) => Err(Error::invalid()),
            },
            None => Err(Error::invalid()),
        }
    }

    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        if let Ok(value) = HeaderValue::from_str(&self.0) {
            values.extend(std::iter::once(value))
        }
    }
}

fn user_agent() -> String {
    let mut target_os = target_os();

    if target_os.is_empty() {
        target_os = "uniffi".to_owned();
    }

    if &target_os == "ios" {
        target_os = "iOS".to_owned();
    } else {
        target_os = target_os[0..1].to_uppercase() + &target_os[1..];
    }
    format!("{}/{}", target_os, VERSION)
}
