use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
};

use regex::Regex;
use serde_json::json;

use windows::{core::PCWSTR, Win32::System::Diagnostics::Debug::*};

fn main() -> Result<(), Box<dyn Error>> {
    let mut text = String::new();
    let mut file_in = File::open("./rdata.txt")?;
    file_in.read_to_string(&mut text)?;

    let mut gen_type = "";
    let mut gen_methods: Vec<String> = vec![];

    let re1 = Regex::new(r"dq offset (\?[^ ;\n\r]+)")?;
    for it in re1.find_iter(&text) {
        let its = windows::core::HSTRING::from(&it.as_str()[10..]);

        let mut s_bytes: [u16; 300] = [0; 300];
        let s_size = unsafe {
            UnDecorateSymbolNameW(
                PCWSTR::from_raw(its.as_ptr()),
                s_bytes.as_mut_slice(),
                UNDNAME_NAME_ONLY,
            )
        };
        let mut s_string = String::from_utf16_lossy(&s_bytes);
        s_string.truncate(s_size as usize);
        gen_methods.push(s_string.clone());
    }

    let re1_1 = Regex::new(r"([^:\n]+)$")?;
    for it in gen_methods.iter_mut() {
        if let Some(s) = re1_1.find(&it) {
            *it = s.as_str().to_string();
        }
    }

    let re2 = Regex::new(r"const ([\w@:]+)::`vftable'")?;
    if let Some(it) = re2.find(&text) {
        let s = it.as_str();
        gen_type = &s[6..(s.len() - 11)];
    }

    let result = json!({
        "type": gen_type,
        "methods": gen_methods
    });

    let mut file_out = File::create("./rdata.json")?;
    file_out.write_all(serde_json::to_string_pretty(&result)?.as_bytes())?;
    drop(file_out);

    println!("Done.");

    Ok(())
}
