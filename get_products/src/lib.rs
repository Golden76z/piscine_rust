pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let mut vec: Vec<usize> = Vec::new();

    if arr.len() <= 1 {
        return vec;
    }

    for i in 0..arr.len() {
        let mut temp = 1;
        for j in 0..arr.len() {
            if j != i {
                temp *= arr[j];
            }
        }
        vec.push(temp);
    }

    vec
}
