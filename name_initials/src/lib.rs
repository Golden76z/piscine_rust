pub fn initials(names: Vec<&str>) -> Vec<String> {
    // Vec that will contains Initials strings
    let mut initials: Vec<String> = vec![];

    // Ranging over the names vec
    for name in names {
        let mut new_word = String::new();

        // Ranging over the names to get the name and surname initials
        for word in name.split_whitespace() {
            // Getting the first character
            let first_char = word.chars().nth(0).unwrap();

            new_word.push(first_char);
            new_word.push('.');
            new_word.push(' ');
        }
        // Adding the string to the vec without the whitespace at the end
        initials.push(new_word.trim().to_string());
    }

    initials
}
