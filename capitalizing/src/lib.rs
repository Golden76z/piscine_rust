pub fn capitalize_first(input: &str) -> String {
    let mut result = String::new();
    let vec: Vec<char> = input.chars().collect();

    // Boolean to keep track of the first value
    let mut first_char = true;

    // Ranging over the characters
    for char in vec {
        // Capitalize the first char and change boolean value
        if first_char {
            result.push_str(&char.to_ascii_uppercase().to_string());
            first_char = false
        } else {
            result.push_str(&char.to_string());
        }
    }

    result
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    let vec: Vec<char> = input.chars().collect();

    // Boolean to keep track of the first value
    let mut first_char = true;

    for char in vec {
        if char.is_whitespace() {
            first_char = true;
            result.push_str(&char.to_string());
        } else if char.to_string() != " " && first_char {
            result.push_str(&char.to_ascii_uppercase().to_string());
            first_char = false;
        } else {
            result.push_str(&char.to_string());
        }
    }

    result
}

pub fn change_case(input: &str) -> String {
    let mut result = String::new();
    let vec: Vec<char> = input.chars().collect();

    for char in vec {
        if char.is_uppercase() {
            result.push_str(&char.to_ascii_lowercase().to_string());
        } else if char.is_lowercase() {
            result.push_str(&char.to_ascii_uppercase().to_string());
        } else {
            result.push_str(&char.to_string());
        }
    }

    result
}
