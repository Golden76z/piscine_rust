use std::collections::HashMap;

pub fn arrange_phrase(phrase: &str) -> String {
    // Declaring a map - key, value pairs
    let mut result: HashMap<usize, String> = HashMap::new();

    // Ranging over the words seperated by a white space
    for word in phrase.split_whitespace() {
        // Declaring variables for the index and the word without the digit
        let mut index: usize = 0;
        let mut single_word = String::new();

        // Ranging over the word
        for char in word.chars() {
            // Checking if we spotted the index
            if char >= '0' && char <= '9' {
                index = char.to_digit(10).unwrap() as usize;
            } else {
                single_word.push(char);
            }
        }
        // Inserting the word in the map at the specific index
        result.insert(index, single_word);
    }

    // Sort by index and join
    let mut sorted_words: Vec<_> = result.into_iter().collect();
    sorted_words.sort_by_key(|&(index, _)| index);
    sorted_words
        .into_iter()
        .map(|(_, word)| word)
        .collect::<Vec<_>>()
        .join(" ")
}
