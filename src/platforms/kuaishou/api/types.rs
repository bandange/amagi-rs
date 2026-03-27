use serde_json::Value;

/// GraphQL request descriptor used by public Kuaishou API builders.
#[derive(Debug, Clone, PartialEq)]
pub struct KuaishouGraphqlRequest {
    /// Internal request kind label.
    pub request_type: String,
    /// GraphQL endpoint URL.
    pub url: String,
    /// JSON body sent to the GraphQL endpoint.
    pub body: Value,
}
