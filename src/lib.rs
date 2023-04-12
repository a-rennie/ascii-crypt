use crate::DecodeError::UnknownError;
use std::error::Error;
use std::num::IntErrorKind;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EncodeError {
    #[error("Encode function does not support unicode chars (> U+255). Invalid char: {0}")]
    CharNotAscii(char),
}

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("Input invalid (length not divisible by 3)")]
    InvalidLength,
    #[error("Input invalid (not all numbers). Invalid char is in this string: {0}")]
    UnexpectedCharIn(String),
    #[error("Unknown error ({0})")]
    UnknownError(Box<dyn Error>),
}

pub fn encode(inword: &str) -> Result<String, EncodeError> {
    let mut outword = String::new();
    for char in inword.chars() {
        let char = char as u32;
        if char > 255 {
            return Err(EncodeError::CharNotAscii(char::from_u32(char).expect("This shouldn't happen as unicode characters cannot exceed 32 bits in size in any case")));
        }
        outword += &format!("{:0>3}", &char.to_string());
    }
    Ok(outword)
}

pub fn decode(inword: &str) -> Result<String, DecodeError> {
    let mut outword = String::new();

    if inword.len() as f64 % 3.0 != 0.0 {
        return Err(DecodeError::InvalidLength);
    }
    for i in 0..inword.len() / 3 {
        let midword: u8 = match inword[3 * i..3 * i + 3].parse() {
            Ok(ascii) => ascii,
            Err(err) => {
                return match err.kind() {
                    IntErrorKind::InvalidDigit => Err(DecodeError::UnexpectedCharIn(
                        inword[3 * i..3 * i + 3].to_owned(),
                    )),
                    _ => Err(UnknownError(Box::new(err))),
                }
            }
        };
        outword.push(midword as char)
    }
    Ok(outword)
}
