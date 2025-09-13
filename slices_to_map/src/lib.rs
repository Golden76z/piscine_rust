use std::{collections::HashMap, hash::Hash};

pub fn slices_to_map<'a, T: Eq + Hash, U>(
    first: &'a [T],
    second: &'a [U],
) -> HashMap<&'a T, &'a U> {
    let min = first.len().min(second.len());
    let mut map: HashMap<&'a T, &'a U> = HashMap::new();

    for i in 0..min {
        map.insert(&first[i], &second[i]);
    }

    map
}
