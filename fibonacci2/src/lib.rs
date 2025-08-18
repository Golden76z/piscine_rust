// Function to return the fibonacci number of a given number
pub fn fibonacci(num: u32) -> u32 {
    // Storing the 2 first values
    let mut vec = vec![0, 1];
    // Checking if the num is 0
    if num == 0 {
        0
    } else if num == 2 {
        // Checking if num is already in our vec range
        1
    } else {
        for n in 2..num + 1 {
            let index = n as usize;
            vec.push(vec[index - 2] + vec[index - 1]);
        }
        let index = num as usize;
        vec[index]
    }
}
