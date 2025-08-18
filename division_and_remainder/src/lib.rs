pub fn divide(x: i32, y: i32) -> (i32, i32) {
    if y == 0 {
        panic!("ERROR: attempt to divide by 0")
    }
    let result = x / y;
    let remainder = x % y;

    (result, remainder)
}
