use std::io; // brang the `io` input/output library into scope from stadard library `std`


fn main() {
    println!("Guess The Number!");

    println!("Please input your Guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("you guessed: {guess}")
}
