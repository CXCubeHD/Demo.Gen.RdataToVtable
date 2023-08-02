#[cfg(feature = "generate_cpp")]
mod cpp;
#[cfg(feature = "generate_rs")]
mod rs;

#[cfg(feature = "generate_cpp")]
use cpp::*;
#[cfg(feature = "generate_rs")]
use rs::*;

use std::{error::Error, fmt::Display};

use r2v_parser::Gen;

pub enum GenerateType {
    #[cfg(feature = "generate_cpp")]
    Cpp,

    #[cfg(feature = "generate_rs")]
    Rs,
}

#[derive(Debug)]
pub enum GenerateError {
    InvalidType,
}

impl Error for GenerateError {}
impl Display for GenerateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            GenerateError::InvalidType => {
                write!(f, "Could not generate with the provided gen_type")
            }
            _ => write!(f, "Unknown error"),
        }
    }
}

pub fn generate(gen: &Gen, gen_type: GenerateType) -> Result<String, Box<dyn Error>> {
    match gen_type {
        #[cfg(feature = "generate_cpp")]
        GenerateType::Cpp => generate_cpp(gen),

        #[cfg(feature = "generate_rs")]
        GenerateType::Rs => generate_rs(gen),

        _ => Err(Box::new(GenerateError::InvalidType)),
    }
}
