use string_permutation::*;

fn main() {
    let word = "thought";
    let word1 = "thougth";

    let word2 = "avcde";
    let word3 = "edbca";

    println!(
        "Is {:?} a permutation of {:?}? = {}",
        word,
        word1,
        is_permutation(word, word1)
    );

    println!(
        "Is {:?} a permutation of {:?}? = {}",
        word2,
        word3,
        is_permutation(word2, word3)
    );
}
