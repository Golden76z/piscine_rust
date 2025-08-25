#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

fn reverse_letter(c: char) -> Option<char> {
    if c.is_ascii_lowercase() {
        Some((b'a' + (b'z' - c as u8)) as char)
    } else if c.is_ascii_uppercase() {
        Some((b'A' + (b'Z' - c as u8)) as char)
    } else {
        None
    }
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let vec_og: Vec<char> = original.chars().collect();
    let vec_ciph: Vec<char> = ciphered.chars().collect();

    let mut exp_ciphered = String::new();
    for char in vec_og.iter() {
        if char.is_alphabetic() {
            exp_ciphered.push_str(&reverse_letter(*char).unwrap().to_string());
        } else {
            exp_ciphered.push_str(&char.to_string());
        }
    }

    if vec_og.len() != vec_ciph.len() || ciphered != exp_ciphered {
        let result = CipherError {
            expected: exp_ciphered,
        };
        return Err(result);
    } else {
        return Ok(());
    }
}
