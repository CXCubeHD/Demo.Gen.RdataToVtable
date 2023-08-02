use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Gen {
    pub r#type: String,
    pub scoped_type: String,
    pub methods: Vec<GenMethod>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct GenMethod {
    pub index: usize,
    pub symbol: String,
    pub undecorated_symbol: String,
    pub cleaned_symbol: String,
    pub name: String,
    pub scoped_name: String,
    pub parameter_types: Vec<String>,
    pub return_type: String,
    pub visibility: String,
}