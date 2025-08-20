use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    // Frequency maps the the 2 strings
    let mut map_s1 = HashMap::new();
    let mut map_s2 = HashMap::new();

    // Count frequencies on first string
    for item in s1.chars() {
        *map_s1.entry(item).or_insert(0) += 1;
    }

    // Count frequencies on second string
    for item in s2.chars() {
        *map_s2.entry(item).or_insert(0) += 1;
    }

    // Returning the comparison of the 2 maps
    map_s1 == map_s2
}
