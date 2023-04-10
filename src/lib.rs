use std::num::IntErrorKind;

pub fn encode(inword: &str) -> Result<String, String> {
    let mut outword = String::new();
    for char in inword.chars(){
        let char = char as u32;
        if char > 255{
            return Err(String::from("Encode function does not support unicode chars (> U+255)"))
        }
        outword += &format!("{:0>3}", &char.to_string());
    }
    Ok(outword)
}

pub fn decode(inword: &str) -> Result<String, String> {
    let mut outword = String::new();

    if inword.len() as f64 % 3.0 != 0.0 {
        return Err(String::from("Input invalid (length not divisible by 3)"))
    }
    for i in 0..inword.len()/3 {
        let midword: u8 = match inword[3*i .. 3*i + 3].parse(){
            Ok(ascii) => ascii,
            Err(err) => return match err.kind() {
                IntErrorKind::InvalidDigit => Err(String::from("Input invalid (not all numbers)")),
                _ => Err(format!("unknown error: {}, {:?}", err, err.kind())),
            },
        };
        outword.push(midword as char)

    }
    Ok(outword)
}