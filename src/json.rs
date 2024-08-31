#[derive(Debug)]
pub enum JasonValue {
    Object(Vec<(String, JasonValue)>),
    Array(Vec<JasonValue>),
    String(String),
    Bool(bool),
    Number(f64),
}
