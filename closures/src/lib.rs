pub fn first_fifty_even_square() -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();
    let mut index: i32 = 2;

    while index <= 100 {
        vec.push(index * index);
        index += 2;
    }

    vec
}
