pub fn edit_distance(source: &str, target: &str) -> usize {
    // Getting the length of the 2 strings
    let s1_len = source.chars().count();
    let s2_len = target.chars().count();

    // 2 dimensionnal array to store needed operations
    let mut vec = vec![vec![0; s2_len + 1]; s1_len + 1];

    // Filling the first row
    for index in 0..s1_len + 1 {
        vec[index][0] = index;
    }

    // Filling the first column
    for index in 0..s2_len + 1 {
        vec[0][index] = index;
    }

    // Compute edit distance using dynamic programming
    for i in 1..=s1_len {
        for j in 1..=s2_len {
            let cost = if source.chars().nth(i - 1) == target.chars().nth(j - 1) {
                0
            } else {
                1
            };

            vec[i][j] = (vec[i - 1][j] + 1) // Deletion
                .min(vec[i][j - 1] + 1) // Insertion
                .min(vec[i - 1][j - 1] + cost); // Substitution
        }
    }

    // Debug print of the 2 dimensionnal vec
    // for row in &vec {
    //     for &value in row {
    //         print!("{:3} ", value);
    //     }
    //     println!()
    // }

    vec[s1_len][s2_len]
}
