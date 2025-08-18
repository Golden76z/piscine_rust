pub fn rev_str(input: &str) -> String {
    // Converting the input into an array
    let chars: Vec<char> = input.chars().collect();

    // Declaring a result variable to store what we will return
    let mut result = String::new();

    // Ranging backwards on the input string
    for index in (0..chars.len()).rev() {
        let char = chars[index];
        result.push(char)
    }
    result
}
