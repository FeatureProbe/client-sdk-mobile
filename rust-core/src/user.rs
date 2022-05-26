use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct FPUser {
    pub key: String,
    attrs: HashMap<String, String>,
}

impl FPUser {
    pub fn new<T: Into<String>>(key: T) -> Self {
        let key = key.into();
        FPUser {
            key,
            ..Default::default()
        }
    }

    pub fn with<T: Into<String>>(mut self, k: T, v: T) -> Self {
        self.attrs.insert(k.into(), v.into());
        self
    }

    pub fn with_attrs(mut self, attrs: impl Iterator<Item = (String, String)>) -> Self {
        self.attrs.extend(attrs);
        self
    }

    pub fn get(&self, k: &str) -> Option<&String> {
        self.attrs.get(k)
    }

    pub fn get_all(&self) -> &HashMap<String, String> {
        &self.attrs
    }

    pub fn as_base64(&self) -> String {
        let json_str = serde_json::to_string(&self).expect("must be valid");
        base64::encode(json_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_with() {
        let u = FPUser::new("key").with("name", "bob").with("phone", "123");
        assert_eq!(u.key, "key");
        assert_eq!(u.get("name"), Some(&"bob".to_owned()));
        assert_eq!(u.get("phone"), Some(&"123".to_owned()));
        assert_eq!(u.get_all().len(), 2);
    }

    #[test]
    fn test_user_with_attrs() {
        let mut attrs: HashMap<String, String> = Default::default();
        attrs.insert("name".to_owned(), "bob".to_owned());
        attrs.insert("phone".to_owned(), "123".to_owned());
        let u = FPUser::new("key").with_attrs(attrs.into_iter());
        assert_eq!(u.get_all().len(), 2);
    }
}
