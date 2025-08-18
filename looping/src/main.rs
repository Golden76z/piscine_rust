use std::io;

fn main() {
    let mut trials = 0;
    loop {
        trials += 1;
        println!(
            "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?"
        );

        // Creating a new empty string
        let mut guess = String::new();

        // Listening for the user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim() == "The letter e" {
            println!("Number of trials: {trials}");
            break;
        }
    }
}
