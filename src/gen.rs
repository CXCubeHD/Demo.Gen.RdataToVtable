use serde::Serialize;

#[derive(Default, Serialize)]
pub struct GenMethod {
    pub symbol: String,
    pub undecorated_symbol: String,
    pub cleaned_symbol: String,
    pub name: String,
    pub scoped_name: String,
    pub parameter_types: Vec<String>,
    pub return_type: String,
    pub visibility: String,
}
