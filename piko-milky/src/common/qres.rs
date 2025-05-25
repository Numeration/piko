use std::str::FromStr;
use fast_str::FastStr;
use serde::{Deserializer, Serializer};
use thiserror::Error;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Scheme {
    Http,
    Https,
    File,
    Base64
}

impl Scheme {
    fn from_str(s: &str) -> Option<Scheme> {
        match s.to_lowercase().as_str() {
            "http" => Some(Scheme::Http),
            "https" => Some(Scheme::Https),
            "file" => Some(Scheme::File),
            "base64" => Some(Scheme::Base64),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Url {
    raw_url: FastStr,
    scheme: Scheme,
}

impl From<Url> for (Scheme, FastStr) {
    fn from(value: Url) -> Self {
        (value.scheme, value.raw_url)
    }
}

impl<'a> From<&'a Url> for (Scheme, &'a FastStr) {
    fn from(value: &'a Url) -> Self {
        (value.scheme, &value.raw_url)
    }
}

#[derive(Error, Debug)]
#[error("Url parse error: {0}")]
pub struct UrlParseError(String);

impl TryFrom<FastStr> for Url {
    type Error = UrlParseError;

    fn try_from(value: FastStr) -> Result<Self, Self::Error> {
        let scheme_end = value.find("://").ok_or_else(|| {
            UrlParseError("Missing entity delimiter (://)".to_string())
        })?;

        let scheme_str = &value[..scheme_end];
        let scheme = Scheme::from_str(scheme_str).ok_or_else(|| {
            UrlParseError(format!("Unsupported URL entity: {}", scheme_str))
        })?;

        Ok(Url {
            raw_url: value,
            scheme,
        })
    }
}

impl TryFrom<String> for Url {
    type Error = UrlParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(FastStr::from(value))
    }
}

impl FromStr for Url {
    type Err = UrlParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.to_owned())
    }
}

impl serde::Serialize for Url {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(&self.raw_url)
    }
}

impl<'de> serde::Deserialize<'de> for Url {
    
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        Url::from_str(&s).map_err(serde::de::Error::custom)
    }
}