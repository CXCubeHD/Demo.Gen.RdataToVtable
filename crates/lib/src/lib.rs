#![allow(unused_imports)]

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn rdata_to_json(rdata: String) -> String {
    let gen = r2v_parser::rdata_to_gen(rdata);
    if gen.is_err() { return "".to_string(); }
    r2v_parser::gen_to_json(&gen.unwrap()).unwrap_or("".to_string())
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn rdata_to_cpp(rdata: String) -> String {
    let gen = r2v_parser::rdata_to_gen(rdata);
    if gen.is_err() { return "".to_string(); }
    r2v_gen::generate(&gen.unwrap(), r2v_gen::GenerateType::Cpp).unwrap_or("".to_string())
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn rdata_to_rs(rdata: String) -> String {
    let gen = r2v_parser::rdata_to_gen(rdata);
    if gen.is_err() { return "".to_string(); }
    r2v_gen::generate(&gen.unwrap(), r2v_gen::GenerateType::Rs).unwrap_or("".to_string())
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn json_to_cpp(json: String) -> String {
    let gen = r2v_parser::json_to_gen(json);
    if gen.is_err() { return "".to_string(); }
    r2v_gen::generate(&gen.unwrap(), r2v_gen::GenerateType::Cpp).unwrap_or("".to_string())
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn json_to_rs(json: String) -> String {
    let gen = r2v_parser::json_to_gen(json);
    if gen.is_err() { return "".to_string(); }
    r2v_gen::generate(&gen.unwrap(), r2v_gen::GenerateType::Rs).unwrap_or("".to_string())
}
