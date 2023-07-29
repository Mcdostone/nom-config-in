use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Type<T> {
    pub prompt: String,
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<T>,
}
