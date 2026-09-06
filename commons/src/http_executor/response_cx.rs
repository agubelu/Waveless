// Waveless
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use crate::*;

/// TODO: add docs.
#[derive(Getters, MutGetters, Default)]
#[getset(get = "pub", get_mut = "pub")]
pub struct ResponseCx {
    status: StatusCode,
    headers: HashMap<CompactString, CompactString>,
    body: Option<BodyValue>,
}

impl ResponseCx {
    pub fn new(headers: HashMap<CompactString, CompactString>, body: Option<BodyValue>) -> Self {
        Self {
            status: StatusCode::OK,
            headers,
            body,
        }
    }

    // Cannot change the default name of `Constructor`.
    pub fn new_with_status(
        status: StatusCode,
        headers: HashMap<CompactString, CompactString>,
        body: Option<BodyValue>,
    ) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }
}

/// TODO: add docs.
pub enum BodyValue {
    Json(serde_json::Value),
    Any(Box<dyn Encode<Output = Bytes> + Send + Sync>),
}
