pub fn delete_and_backspace(s: &mut String) {
    // Creating a clone of s
    let original: Vec<char> = s.chars().collect();

    // Clearing s to rebuild it
    s.clear();

    // Count variable for the +, characters that we will have to ignore
    let mut count = 0;

    // Ranging over the original string
    for &c in &original {
        match c {
            // If we encounter a '-', get rid of the last value of s
            '-' => {
                s.pop();
            }
            '+' => {
                count += 1;
            }
            // If thats a character, check count value
            _ => {
                // If its different from 0, ignore the character and decrement count
                if count > 0 {
                    count -= 1;

                    // Else we add that character to s
                } else {
                    s.push(c);
                }
            }
        }
    }
}

pub fn do_operations(v: &mut [String]) {
    for word in v.iter_mut() {
        if word.contains("+") {
            let nums: Vec<&str> = word.split('+').collect();

            // Making the operation
            let left: i32 = nums[0].parse().expect("not a number");
            let right: i32 = nums[1].parse().expect("not a number");
            let operation = left + right;

            // Changing the value of the vec
            *word = operation.to_string();
        } else if word.contains("-") {
            let nums: Vec<&str> = word.split('-').collect();

            // Making the operation
            let left: i32 = nums[0].parse().expect("not a number");
            let right: i32 = nums[1].parse().expect("not a number");
            let operation = left - right;

            // Changing the value of the vec
            *word = operation.to_string();
        } else {
            panic!("ERROR: invalid operation!")
        }
    }
}
