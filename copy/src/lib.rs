pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    // Calculing the absolute value
    let abs = c.checked_abs().expect("ERROR: overflow");
    let ln: f64 = abs as f64;

    // Calculing the exponential value
    let exp: f64 = c as f64;

    (c, exp.exp(), ln.ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut result = String::new();

    // Ranging over the string without the white spaces
    for part in a.split_whitespace() {
        // Converting that string piece to an float64
        let num: f64 = part.parse().expect("ERROR: not a number");
        let exp_num = num.exp();

        // Appending that exp value to the result string
        result.push_str(&exp_num.to_string());
        result.push_str(" ");
    }

    (a, result.trim().to_string())
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut v: Vec<f64> = Vec::new();
    let dup = &b;

    // Ranging over the vec
    for part in dup {
        // Pushing the logarithm of the absolute value into the v vec
        let abs = part.checked_abs().expect("ERROR: wrong entry");
        let ln: f64 = abs as f64;
        v.push(ln.ln());
    }

    (b, v)
}
