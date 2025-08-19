pub fn str_len(s: &str) -> usize {
    let mut count: usize = 0;

    // Storing the characters individualy into a vec
    let v: Vec<char> = s.chars().collect();

    // Ranging over the vec
    for _ in 0..v.len() {
        count += 1;
    }
    count
}
