pub fn first_subword(s: String) -> String {
    // Converting the input into an array
    let chars: Vec<char> = s.chars().collect();

    // Declaring a result variable to store what we will return
    let mut result = String::new();

    // Ranging over the array
    for index in 0..chars.len() {
        let byte = chars[index] as u32;
        // println!("Byte value {byte}");

        // If the character is not between 97 and 122, break
        if index != 0 && byte < 96 {
            break;
        }

        // Adding the character to the result variable
        let char = chars[index];
        result.push(char);
    }

    // Returning the result
    result
}
