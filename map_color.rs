use std::{
    collections::HashMap,
    fmt::Display,
    hash::{self, Hash}
};

use serde::de::value;
use thiserror::Error;

#[derive(Debug, serde::Deserialize)]
pub struct Record {
    pub block: String,
    pub color_id: String
}

#[derive(Debug)]
pub enum ColorType {
    Single(i8),
    Multiple(HashMap<State, i8>)
}

pub fn find_id_by_states<'a>(map: &'a HashMap<State, i8>, target_state_key: &str, target_state_value: &str) -> Option<&'a i8> {
    map.into_iter().find_map(|(key, value)| {
        let matched = key.inner.iter().any(|(state_key, state_value)| state_key == target_state_key && state_value == target_state_value);

        if matched {
            Some(value)
        } else {
            None
        }
    })
}

#[derive(Debug, Error)]
pub enum StateError {
    #[error("Failed parse State from String {}", .0)]
    StateParseError(String)
}

#[derive(Debug, PartialEq, Eq)]
pub struct State {
    inner: HashMap<String, String>
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut properties = self.inner
        .iter()
        .collect::<Vec<_>>();

        properties.sort_by(|(key_a, _), (key_b, _)| key_a.cmp(key_b)); 

        let properties = properties
            .iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<_>>()
            .join(",");

        write!(
            f,
            "{}",
            properties
        )
    }
}

impl Hash for State {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.to_string().hash(state)
    }
}

impl TryFrom<&String> for State {
    type Error = StateError;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let inner = value
            .split(",")
            .into_iter()
            .map(|value| {
                let split: Vec<_> = value.split("=").collect();
                let Some(key) = split.get(0) else { return Err(StateError::StateParseError(value.to_string())); };
                let Some(value) = split.get(1) else { return Err(StateError::StateParseError(value.to_string())); };

                Ok((key.to_string(), value.to_string()))
            })
            .collect::<Result<HashMap<String, String>, StateError>>()?;

        Ok(State {
            inner
        })
    }
}
