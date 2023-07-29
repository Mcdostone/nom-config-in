use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DefineType<T> {
    pub symbol: String,
    pub value: Option<T>,
}
