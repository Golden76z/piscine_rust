use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let length: f64 = list.len() as f64;
    let mut result: i32 = 0;

    for item in list {
        result += item;
    }

    let convert: f64 = result as f64;
    convert / length
}

pub fn median(list: &[i32]) -> i32 {
    let mut copy = list.to_owned();
    copy.sort();

    if copy.len() % 2 == 0 {
        return (copy[copy.len() / 2] + copy[(copy.len() / 2) - 1]) / 2;
    } else {
        return copy[copy.len() / 2];
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut frequency_map = HashMap::new();

    // Count frequencies
    for &item in list {
        *frequency_map.entry(item).or_insert(0) += 1;
    }

    // Find the value with the highest frequency
    frequency_map
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| value)
        .unwrap_or(0)
}
