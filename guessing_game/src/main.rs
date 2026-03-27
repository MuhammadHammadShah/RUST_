use rand::Rng;
use std::cmp::Ordering;
use std::io; // brang the `io` input/output library into scope from stadard library `std`

fn main() {
    println!("Guess The Number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The Secret Number is: {secret_number}");
    println!("Please input your Guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number");

    println!("you guessed: {guess}");

    // comparing
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small"),
        Ordering::Greater => println!("Too Big"),
        Ordering::Equal => println!("Oh Yeah!, It is theeeeeeeeee Correct."),
    }
}
