pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    s.split_whitespace()
        .filter_map(|token| {
            if token.ends_with('k') {
                let num_str = &token[..token.len() - 1];
                num_str
                    .parse::<f64>()
                    .ok()
                    .map(|f| Box::new((f * 1000.0) as u32))
            } else {
                token.parse::<f64>().ok().map(|f| Box::new(f as u32))
            }
        })
        .collect()
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    a.into_iter().map(|boxed| *boxed).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_boxing() {
        let s = "5.5k 8.9k 32".to_owned();

        let boxed = parse_into_boxed(s);
        println!("Element value: {:?}", boxed[0]);
        println!("Element size: {:?} bytes", mem::size_of_val(&boxed[0]));

        let unboxed = into_unboxed(boxed);
        println!("Element value: {:?}", unboxed[0]);
        println!("Element size: {:?} bytes", mem::size_of_val(&unboxed[0]));

        // As with everything related to regular Rust memory management, both the `Vec` and the `Box`es will be properly dropped when out of scope and freed, ensuring no leaks
    }
}
