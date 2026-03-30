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
    println!("{spaces}");
    let spaces = spaces.len();
    println!("{spaces}");

    // change value type

    let number: u32 = "45".parse().expect("Not a Number"); // u32 is the type annotation.
    println!("{number}");
    
    // overflow check with -release
    // let mut numer: u8 = 255;
    // println!("{numer}");
    // numer = numer + 2;
    // println!("{numer}");
}
