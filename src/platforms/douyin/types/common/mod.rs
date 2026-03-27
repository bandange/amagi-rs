use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer, Serialize, de};
use serde_json::Value;

#[cfg(feature = "cli")]
use clap::ValueEnum;

mod control;
mod media;
mod meta;
mod text;

pub type DouyinExtraFields = BTreeMap<String, Value>;

pub(crate) fn deserialize_null_default_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<Vec<T>>::deserialize(deserializer).map(Option::unwrap_or_default)
}

pub(crate) fn deserialize_null_default_string_vec<'de, D>(
    deserializer: D,
) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let values = Option::<Vec<Value>>::deserialize(deserializer)?.unwrap_or_default();
    values
        .into_iter()
        .map(|value| match value {
            Value::String(value) => Ok(value),
            Value::Number(value) => Ok(value.to_string()),
            Value::Bool(value) => Ok(value.to_string()),
            other => Err(de::Error::custom(format!(
                "expected string-compatible value, got {other}"
            ))),
        })
        .collect()
}

pub use control::{
    DouyinAwemeControl, DouyinCommentPermissionInfo, DouyinReviewResult, DouyinStatistics,
    DouyinStatus,
};
pub use media::{DouyinImageUrl, DouyinSearchImpression, DouyinShareInfo};
pub use meta::{DouyinLogPb, DouyinRawPayload, DouyinResponseMeta};
pub use text::{
    DouyinInlineSuggestWord, DouyinInlineSuggestWordEntry, DouyinInlineSuggestWords,
    DouyinTextExtra,
};

#[cfg_attr(feature = "cli", derive(ValueEnum))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DouyinSearchType {
    #[default]
    General,
    User,
    Video,
}

impl DouyinSearchType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::General => "general",
            Self::User => "user",
            Self::Video => "video",
        }
    }
}
