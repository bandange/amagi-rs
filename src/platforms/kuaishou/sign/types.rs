use std::collections::BTreeMap;

/// Ordered JSON object used by the Kuaishou signing payload.
///
/// The original browser implementation relies on `JSON.stringify`, whose output
/// preserves insertion order for object keys. This wrapper keeps that order so
/// Rust can reproduce the same sign input exactly.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct KuaishouJsonObject(Vec<(String, KuaishouJsonValue)>);

impl KuaishouJsonObject {
    /// Create an empty ordered object.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a key-value pair to the object.
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<KuaishouJsonValue>) {
        self.0.push((key.into(), value.into()));
    }

    /// Return whether the object contains no entries.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Return the ordered entries of the object.
    pub fn entries(&self) -> &[(String, KuaishouJsonValue)] {
        &self.0
    }
}

impl<K, V> From<Vec<(K, V)>> for KuaishouJsonObject
where
    K: Into<String>,
    V: Into<KuaishouJsonValue>,
{
    fn from(value: Vec<(K, V)>) -> Self {
        value.into_iter().collect()
    }
}

impl<K, V> FromIterator<(K, V)> for KuaishouJsonObject
where
    K: Into<String>,
    V: Into<KuaishouJsonValue>,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut object = Self::new();

        for (key, value) in iter {
            object.insert(key, value);
        }

        object
    }
}

impl IntoIterator for KuaishouJsonObject {
    type Item = (String, KuaishouJsonValue);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a KuaishouJsonObject {
    type Item = &'a (String, KuaishouJsonValue);
    type IntoIter = std::slice::Iter<'a, (String, KuaishouJsonValue)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

/// Minimal JSON value used by the Kuaishou sign input serializer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KuaishouJsonValue {
    /// JSON `null`.
    Null,
    /// JSON boolean.
    Bool(bool),
    /// JSON number rendered from the stored string.
    Number(String),
    /// JSON string.
    String(String),
    /// JSON array.
    Array(Vec<KuaishouJsonValue>),
    /// JSON object.
    Object(KuaishouJsonObject),
}

impl KuaishouJsonValue {
    /// Create a number value from an already-normalized textual representation.
    pub fn number(value: impl Into<String>) -> Self {
        Self::Number(value.into())
    }
}

impl From<bool> for KuaishouJsonValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<String> for KuaishouJsonValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for KuaishouJsonValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<i32> for KuaishouJsonValue {
    fn from(value: i32) -> Self {
        Self::Number(value.to_string())
    }
}

impl From<i64> for KuaishouJsonValue {
    fn from(value: i64) -> Self {
        Self::Number(value.to_string())
    }
}

impl From<isize> for KuaishouJsonValue {
    fn from(value: isize) -> Self {
        Self::Number(value.to_string())
    }
}

impl From<u32> for KuaishouJsonValue {
    fn from(value: u32) -> Self {
        Self::Number(value.to_string())
    }
}

impl From<u64> for KuaishouJsonValue {
    fn from(value: u64) -> Self {
        Self::Number(value.to_string())
    }
}

impl From<usize> for KuaishouJsonValue {
    fn from(value: usize) -> Self {
        Self::Number(value.to_string())
    }
}

impl From<f64> for KuaishouJsonValue {
    fn from(value: f64) -> Self {
        Self::Number(value.to_string())
    }
}

impl From<KuaishouJsonObject> for KuaishouJsonValue {
    fn from(value: KuaishouJsonObject) -> Self {
        Self::Object(value)
    }
}

impl From<Vec<KuaishouJsonValue>> for KuaishouJsonValue {
    fn from(value: Vec<KuaishouJsonValue>) -> Self {
        Self::Array(value)
    }
}

/// Structured payload used to generate `__NS_hxfalcon`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct KuaishouHxfalconPayload {
    /// Canonical signing path used by the algorithm.
    pub url: String,
    /// Query parameters that participate in signing.
    pub query: BTreeMap<String, String>,
    /// Form parameters that participate in signing.
    pub form: BTreeMap<String, String>,
    /// JSON request body used by the signing algorithm.
    pub request_body: KuaishouJsonObject,
}

/// HTTP method used by a Kuaishou `live_api` request description.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KuaishouLiveApiMethod {
    /// `GET`.
    #[default]
    Get,
    /// `POST`.
    Post,
}

/// Structured Kuaishou `live_api` request description.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KuaishouLiveApiRequest {
    /// Internal request kind label.
    pub request_type: String,
    /// Actual request URL.
    pub url: String,
    /// HTTP method used by the request.
    pub method: KuaishouLiveApiMethod,
    /// Whether the caller expects the request to be signed.
    pub requires_sign: bool,
    /// Optional canonical sign path when it differs from the public path.
    pub sign_path: Option<String>,
    /// Optional JSON body for the request.
    pub body: KuaishouJsonObject,
}

impl KuaishouLiveApiRequest {
    /// Create a new Kuaishou `live_api` request description.
    pub fn new(request_type: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            request_type: request_type.into(),
            url: url.into(),
            method: KuaishouLiveApiMethod::Get,
            requires_sign: true,
            sign_path: None,
            body: KuaishouJsonObject::new(),
        }
    }

    /// Override the HTTP method.
    pub fn with_method(mut self, method: KuaishouLiveApiMethod) -> Self {
        self.method = method;
        self
    }

    /// Override whether the request requires signing.
    pub fn with_requires_sign(mut self, requires_sign: bool) -> Self {
        self.requires_sign = requires_sign;
        self
    }

    /// Attach a canonical sign path.
    pub fn with_sign_path(mut self, sign_path: impl Into<String>) -> Self {
        self.sign_path = Some(sign_path.into());
        self
    }

    /// Attach an ordered JSON body.
    pub fn with_body(mut self, body: KuaishouJsonObject) -> Self {
        self.body = body;
        self
    }
}

/// Result of signing a Kuaishou `live_api` request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KuaishouLiveApiSignature {
    /// Signed request URL with `__NS_hxfalcon`.
    pub url: String,
    /// Extra request headers such as `kww`.
    pub headers: BTreeMap<String, String>,
    /// Final `__NS_hxfalcon` value.
    pub sign_result: String,
    /// Raw sign input used to produce the signature.
    pub sign_input: String,
    /// Runtime `caver` value used for signing.
    pub cat_version: String,
}

/// Runtime-generated `__NS_hxfalcon` materials derived from a payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KuaishouGeneratedHxfalcon {
    /// Final `__NS_hxfalcon` value.
    pub sign_result: String,
    /// Raw sign input used to produce the signature.
    pub sign_input: String,
    /// Runtime `caver` value used for signing.
    pub cat_version: String,
}

/// Process-wide runtime state that mirrors the browser-side pure sign runtime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KuaishouPureRuntimeState {
    /// Current `caver` value.
    pub cat_version: String,
    /// Current request counter.
    pub count: u32,
    /// Startup timestamp used by `$HE_`.
    pub startup_random: u64,
}

/// Minimal `window.SECS` equivalent used by `HUDR_`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct KuaishouSecsState {
    /// `SECS.c`.
    pub c: Option<u32>,
    /// `SECS.s`.
    pub s: Option<String>,
}

/// Context required to derive the `HUDR_` segment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KuaishouHudrContext {
    /// Current signature counter.
    pub count: u32,
    /// Number of page scripts observed by the runtime.
    pub script_count: Option<u32>,
    /// Optional `SECS` state.
    pub secs: Option<KuaishouSecsState>,
}

/// Intermediate `HUDR_` result and debug-friendly state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KuaishouHudrResult {
    /// `HUDR_` body without the prefix.
    pub body: String,
    /// Full `HUDR_...` segment.
    pub full: String,
    /// `infoCache` bytes embedded in the payload.
    pub info_cache: Vec<u8>,
    /// Masked payload before ChaCha encryption.
    pub masked_payload: Vec<u8>,
    /// Next counter value after this HUDR result.
    pub next_count: u32,
}

/// Context required to derive the `$HE_` segment.
#[derive(Debug, Clone, PartialEq)]
pub struct KuaishouHeContext {
    /// Current signature counter.
    pub count: u32,
    /// `HUDR_` body without the prefix.
    pub hudr_body: String,
    /// Random value equivalent to `Math.random()`.
    pub random_value: f64,
    /// Hxfalcon sign input.
    pub sign_input: String,
    /// Process startup timestamp.
    pub startup_random: u64,
    /// Current request timestamp.
    pub timestamp: u64,
}

/// Intermediate `$HE_` result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KuaishouHeResult {
    /// Final `$HE_` hex payload.
    pub final_hex: String,
    /// Four-byte hash field used inside `$HE_`.
    pub hash_field_hex: String,
    /// Hex prefix before the final LRC checksum.
    pub pre_hex: String,
}

/// Context required to derive the complete pure signature result.
#[derive(Debug, Clone, PartialEq)]
pub struct KuaishouPureSignContext {
    /// Current signature counter.
    pub count: u32,
    /// Number of page scripts observed by the runtime.
    pub script_count: Option<u32>,
    /// Optional `SECS` state.
    pub secs: Option<KuaishouSecsState>,
    /// Random value equivalent to `Math.random()`.
    pub random_value: f64,
    /// Hxfalcon sign input.
    pub sign_input: String,
    /// Process startup timestamp.
    pub startup_random: u64,
    /// Current request timestamp.
    pub timestamp: u64,
}

/// Complete pure signature result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KuaishouPureSignResult {
    /// `HUDR_` intermediate result.
    pub hudr: KuaishouHudrResult,
    /// `$HE_` intermediate result.
    pub he: KuaishouHeResult,
    /// Final `HUDR_...$HE_...` signature.
    pub sign_result: String,
}
