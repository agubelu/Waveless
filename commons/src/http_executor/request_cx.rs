// Waveless
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use crate::*;

use endpoint::*;

/// TODO: add docs.
#[derive(Getters, MutGetters, Debug)]
#[getset(get = "pub", get_mut = "pub")]
pub struct RequestCx {
    /// NOTE: make sure not to consume the request body if you want a streaming request.
    pub request: Request<BoxBody<ConnBytes, eyre::Error>>,
    pub method: HttpMethod,
    pub request_params: HashMap<CompactString, ParamValue>,
    pub endpoint: Endpoint,
}

/// Defines whether a request parameter came from the client or was injected by the runtime.
/// NOTE: request params from the client will be overwriten by the executor if they have the same key.
/// TODO: fix the previous behaviour?
#[derive(Clone, Debug)]
pub enum ParamValue {
    Internal(CompactString),
    Client(Option<CompactString>),
}
