mod feature_probe;
mod sync;
mod user;

use std::collections::HashMap;

pub use crate::user::FPUser;
pub use feature_probe::{FPConfig, FeatureProbe};
use headers::{Error, Header, HeaderName, HeaderValue};
use http::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
pub use url::Url;

pub type Repository = HashMap<String, FPDetail<Value>>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
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
