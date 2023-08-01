mod gen;

use crate::gen::*;

use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
};

use regex::Regex;
use serde_json::json;

#[cfg(target_os = "windows")]
use windows::{core::PCWSTR, Win32::System::Diagnostics::Debug::*};

#[cfg(not(target_os = "windows"))]
use msvc_demangler::DemangleFlags;

fn main() -> Result<(), Box<dyn Error>> {
    let mut text = String::new();
    let mut file_in = File::open("./rdata.txt")?;
    file_in.read_to_string(&mut text)?;

    let mut gen_type = "";
    let mut gen_methods: Vec<GenMethod> = vec![];

    /* Extract methods */
    let re1 = Regex::new(r"dq offset (\?[^ ;\n\r]+)")?;
    for it in re1.find_iter(&text) {
        const NAME_LENGTH_SIZE: usize = 500;
        const NAME_LENGTH_LIMIT: usize = 490;
        let mut s_bytes: [u16; NAME_LENGTH_SIZE];
        let mut s_size: usize;

        #[cfg(target_os = "windows")]
        {
            let its = windows::core::HSTRING::from(&it.as_str()[10..]);

            /* Undecorated Symbol */
            s_bytes = [0; NAME_LENGTH_SIZE];
            s_size = unsafe {
                UnDecorateSymbolNameW(
                    PCWSTR::from_raw(its.as_ptr()),
                    s_bytes.as_mut_slice(),
                    UNDNAME_COMPLETE,
                ) as usize
            };

            let mut undecorated_symbol = String::from_utf16_lossy(&s_bytes);
            undecorated_symbol.truncate(s_size);
            if undecorated_symbol.len() > NAME_LENGTH_LIMIT {
                undecorated_symbol = String::new();
            }

            /* Cleaned Symbol */
            s_bytes = [0; NAME_LENGTH_SIZE];
            s_size = unsafe {
                UnDecorateSymbolNameW(
                    PCWSTR::from_raw(its.as_ptr()),
                    s_bytes.as_mut_slice(),
                    UNDNAME_NO_THROW_SIGNATURES
                        | UNDNAME_NO_ACCESS_SPECIFIERS
                        | UNDNAME_NO_ALLOCATION_MODEL
                        | UNDNAME_NO_SPECIAL_SYMS
                        | UNDNAME_NO_ALLOCATION_LANGUAGE
                        | UNDNAME_NO_MS_KEYWORDS
                        | UNDNAME_NO_MEMBER_TYPE,
                ) as usize
            };

            let mut cleaned_symbol = String::from_utf16_lossy(&s_bytes);
            cleaned_symbol.truncate(s_size);
            if cleaned_symbol.len() > NAME_LENGTH_LIMIT {
                cleaned_symbol = String::new();
            }

            /* Name Only */
            s_bytes = [0; NAME_LENGTH_SIZE];
            s_size = unsafe {
                UnDecorateSymbolNameW(
                    PCWSTR::from_raw(its.as_ptr()),
                    s_bytes.as_mut_slice(),
                    UNDNAME_NAME_ONLY,
                ) as usize
            };

            let mut name_only = String::from_utf16_lossy(&s_bytes);
            name_only.truncate(s_size);
            if name_only.len() > NAME_LENGTH_LIMIT {
                name_only = String::new();
            }

            gen_methods.push(GenMethod {
                symbol: its.to_string(),
                undecorated_symbol: undecorated_symbol.clone(),
                cleaned_symbol: cleaned_symbol,
                name: name_only.clone(),
                scoped_name: name_only,
                ..Default::default()
            });
        }

        #[cfg(not(target_os = "windows"))]
        {
            let its = &it.as_str()[10..];

            let mut undecorated_symbol = msvc_demangler::demangle(its, DemangleFlags::COMPLETE);
            if undecorated_symbol.len() > NAME_LENGTH_LIMIT {
                undecorated_symbol = String::new();
            }

            let mut cleaned_symbol = msvc_demangler::demangle(its, DemangleFlags::NO_ACCESS_SPECIFIERS
                | DemangleFlags::NO_MS_KEYWORDS
                | DemangleFlags::NO_MEMBER_TYPE);
            cleaned_symbol.truncate(s_size);
            if cleaned_symbol.len() > NAME_LENGTH_LIMIT {
                cleaned_symbol = String::new();
            }

            let mut name_only = msvc_demangler::demangle(its, DemangleFlags::NAME_ONLY);
            name_only.truncate(s_size);
            if name_only.len() > NAME_LENGTH_LIMIT {
                name_only = String::new();
            }

            gen_methods.push(GenMethod {
                symbol: its.to_string(),
                undecorated_symbol: undecorated_symbol.clone(),
                cleaned_symbol: cleaned_symbol,
                name: name_only.clone(),
                scoped_name: name_only,
                ..Default::default()
            });
        }
    }

    /* Filter method name */
    let re1_1 = Regex::new(r"([^:\n]+)$")?;
    for it in gen_methods.iter_mut() {
        if let Some(s) = re1_1.find(&it.name) {
            it.name = s.as_str().to_string();
        }
    }

    /* Extract parameters */
    let re1_2 = Regex::new(r"\((.*?)\)")?;
    for it in gen_methods.iter_mut() {
        if let Some(s) = re1_2.find(&it.cleaned_symbol) {
            let s_parameters = s.as_str()[1..(s.len() - 1)].to_string();

            let mut parameters: Vec<&str> = s_parameters.split(",").collect::<Vec<_>>();

            if parameters.first().unwrap_or(&"void") == &"void" {
                parameters = Vec::new()
            }

            let mut combined_parameters = Vec::<String>::new();
            let mut temp_parameter = String::new();
            let mut angle_brackets_count: i32 = 0;

            for p in parameters {
                let c1 = p.match_indices("<").collect::<Vec<_>>().len() as i32;
                let c2 = p.match_indices(">").collect::<Vec<_>>().len() as i32;

                if angle_brackets_count == 0 {
                    temp_parameter += p;
                    if c1 > c2 {
                        angle_brackets_count += c1 - c2;
                    }
                } else {
                    temp_parameter += ",";
                    temp_parameter += p;
                    angle_brackets_count += c1 - c2;
                }

                if angle_brackets_count == 0 {
                    combined_parameters.push(temp_parameter);
                    temp_parameter = String::new();
                }
            }

            it.parameter_types =
                Vec::from_iter(combined_parameters.iter().map(|p| p.trim().to_string()));
        }
    }

    /* Extract return type */
    for it in gen_methods.iter_mut() {
        if let Some(s) = it
            .cleaned_symbol
            .find(format!(" {}(", it.scoped_name.as_str()).as_str())
        {
            it.return_type = it.cleaned_symbol[..s].to_string();
        }
    }

    /* Extract visibility */
    let re1_4 = Regex::new(r"(^(?:public|private|protected)):")?;
    for it in gen_methods.iter_mut() {
        if let Some(s) = re1_4.find(&it.undecorated_symbol) {
            it.visibility = s.as_str()[..(s.len() - 1)].to_string();
        }
    }

    /* Get type name */
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
