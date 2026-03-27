use serde::Deserialize;

/// Optional pagination query for Kuaishou user work lists.
#[derive(Debug, Default, Deserialize)]
pub struct KuaishouWorkListQuery {
    /// Optional pagination cursor.
    pub pcursor: Option<String>,
    /// Optional page size.
    pub count: Option<u32>,
}
