fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The seconds  are: {}", THREE_HOURS_IN_SECONDS);
    print!("\n");

    let y = 5;
    println!("The value of y is: {}", y);
    let y = 5 * 2;
    println!("The value of y is: {}", y);
    print!("\n");
    {
        let y = 5 + 100;
        println!("The value of y is: {}", y);
    }
    println!("The value of y is: {}", y);

    // shadowing
    let spaces = "    ";
    let spaces = spaces.len();
    println!("{spaces}");
}
