use std::default;

#[derive(Default, Clone)]
pub enum ConvertOptionVoidType {
    #[default]
    RsVoid,
    CVoid,
}

#[derive(Default, Clone)]
pub struct ConvertOptions {
    pub void_type: ConvertOptionVoidType,
    pub comment_hints: bool,
}

pub fn convert_gen_type_native(ty: String, options: ConvertOptions) -> Option<String> {
    let r = match ty.trim() {
        "char" => "i8",
        "short" => "i16",
        "int" => "i32",
        "long" => "i32",
        "long long" => "i64",

        "unsigned char" => "u8",
        "unsigned short" => "u16",
        "unsigned int" => "u32",
        "unsigned long" => "u32",
        "unsigned long long" => "u64",

        "int8_t" => "i8",
        "int16_t" => "i16",
        "int32_t" => "i32",
        "int64_t" => "i64",

        "uint8_t" => "u8",
        "uint16_t" => "u16",
        "uint32_t" => "u32",
        "uint64_t" => "u64",

        "float" => "f32",
        "double" => "f64",

        "void" => match options.void_type {
            ConvertOptionVoidType::RsVoid => "()",
            ConvertOptionVoidType::CVoid => "::std::ffi::c_void",
        },

        "bool" => "bool",

        _ => "",
    };

    if r == "" {
        return None;
    }
    Some(r.to_string())
}

pub fn convert_gen_enum(ty: String, options: ConvertOptions) -> Option<String> {
    let r = if ty.starts_with("enum ") {
        if options.comment_hints {
            format!("/* {} */ {}", ty, "isize")
        } else {
            "isize".to_string()
        }
    } else {
        String::new()
    };

    if r == "" {
        return None;
    }
    Some(r)
}

pub fn convert_gen_type(ty: String) -> Option<String> {
    None
}
