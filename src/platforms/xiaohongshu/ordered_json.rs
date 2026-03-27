use crate::error::AppError;

/// Ordered JSON value used by Xiaohongshu signing so field order matches the source implementation.
#[derive(Debug, Clone, PartialEq)]
pub enum OrderedJson {
    /// JSON `null`.
    Null,
    /// JSON boolean.
    Bool(bool),
    /// JSON number literal stored in textual form.
    Number(String),
    /// JSON string.
    String(String),
    /// JSON array.
    Array(Vec<OrderedJson>),
    /// JSON object preserving insertion order.
    Object(Vec<(String, OrderedJson)>),
}

impl OrderedJson {
    /// Build an ordered JSON object.
    pub fn object(entries: Vec<(impl Into<String>, OrderedJson)>) -> Self {
        Self::Object(
            entries
                .into_iter()
                .map(|(key, value)| (key.into(), value))
                .collect(),
        )
    }

    /// Build a JSON string value.
    pub fn string(value: impl Into<String>) -> Self {
        Self::String(value.into())
    }

    /// Build a JSON number from a signed integer.
    pub fn int(value: i64) -> Self {
        Self::Number(value.to_string())
    }

    /// Build a JSON number from an unsigned integer.
    pub fn uint(value: u64) -> Self {
        Self::Number(value.to_string())
    }

    /// Build a JSON number from a raw literal string.
    pub fn number_literal(value: impl Into<String>) -> Self {
        Self::Number(value.into())
    }

    /// Return the object entries when this value is an object.
    pub fn as_object(&self) -> Option<&[(String, OrderedJson)]> {
        match self {
            Self::Object(entries) => Some(entries),
            _ => None,
        }
    }

    /// Serialize the ordered JSON value using insertion order.
    pub fn to_json_string(&self) -> Result<String, AppError> {
        match self {
            Self::Null => Ok("null".to_owned()),
            Self::Bool(value) => Ok(if *value { "true" } else { "false" }.to_owned()),
            Self::Number(value) => Ok(value.clone()),
            Self::String(value) => serde_json::to_string(value).map_err(Into::into),
            Self::Array(values) => {
                let mut output = String::from("[");
                for (index, value) in values.iter().enumerate() {
                    if index > 0 {
                        output.push(',');
                    }
                    output.push_str(&value.to_json_string()?);
                }
                output.push(']');
                Ok(output)
            }
            Self::Object(entries) => {
                let mut output = String::from("{");
                for (index, (key, value)) in entries.iter().enumerate() {
                    if index > 0 {
                        output.push(',');
                    }
                    output.push_str(&serde_json::to_string(key)?);
                    output.push(':');
                    output.push_str(&value.to_json_string()?);
                }
                output.push('}');
                Ok(output)
            }
        }
    }

    pub(crate) fn to_query_value(&self) -> Result<String, AppError> {
        match self {
            Self::Null => Ok(String::new()),
            Self::Bool(value) => Ok(if *value { "true" } else { "false" }.to_owned()),
            Self::Number(value) => Ok(value.clone()),
            Self::String(value) => Ok(value.clone()),
            Self::Array(values) => {
                let parts = values
                    .iter()
                    .map(OrderedJson::to_query_value)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(parts.join(","))
            }
            Self::Object(_) => self.to_json_string(),
        }
    }
}

impl From<&str> for OrderedJson {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<String> for OrderedJson {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<bool> for OrderedJson {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i64> for OrderedJson {
    fn from(value: i64) -> Self {
        Self::int(value)
    }
}

impl From<u64> for OrderedJson {
    fn from(value: u64) -> Self {
        Self::uint(value)
    }
}
