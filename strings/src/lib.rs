pub fn char_length(s: &str) -> usize {
    let vec: Vec<char> = s.chars().collect();

    let mut count: usize = 0;
    for _ in vec {
        count += 1;
    }

    count
}
