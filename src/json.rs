#[derive(Debug)]
pub enum JsonValue {
    Object(Vec<(String, JsonValue)>),
    Array(Vec<JsonValue>),
    String(String),
    Bool(bool),
    Number(f64),
}
