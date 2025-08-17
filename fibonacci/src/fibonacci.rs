// Function to return the fibonacci number of a given number
pub fn fibonacci(num: i32) -> i32 {
    // Storing the 2 first values
    let mut vec = vec![0, 1];

    // Checking if num is already in our vec range
    if num < 3 {
        let index = num as usize;
        vec[index - 1]
    } else {
        for n in 2..num + 1 {
            let index = n as usize;
            vec.push(vec[index - 2] + vec[index - 1]);
        }
        let index = num as usize;
        vec[index]
    }
}
