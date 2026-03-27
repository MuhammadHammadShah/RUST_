use std::io; // brang the `io` input/output library into scope from stadard library `std`
use rand::Rng;

fn main() {
    println!("Guess The Number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The Secret Number is: {secret_number}");
    println!("Please input your Guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("you guessed: {guess}")
}
