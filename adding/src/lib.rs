pub fn add_curry(base: i32) -> impl Fn(i32) -> i32 {
    move |x| base + x
}
