pub fn factorial(num: u64) -> u64 {
    let mut result = 1;
    for n in 1..num + 1 {
        result *= n;
    }
    result
}
