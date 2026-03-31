// loops ==> loop, while, for               3

fn main() {
    //
    // loop_it_infinitly()
    // continue_condition(false)
    // let value = plus_one(1);
    // println!("from MAIN FUNCTION: {value}");
    // labeled_loop();
    // for_loop();
    while_loop();
}

// while loop

fn while_loop() {
    let mut number: i32 = 10;

    while number >= 0 {
        println!("number: {number}");
        number -= 1;
    }

    println!("Okay! that's it");
}

// for loop
fn for_loop() {
    for number in (1..5).rev() {
        println!("numbers: {number}");
    }
    println!("Bye Bye!")
}

//Loop Labels to Disambiguate Between Multiple Loops
fn labeled_loop() {
    let mut count = 0;
    'outerloop: loop {
        println!("count: {count}");
        let mut remaining: i32 = 10;
        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'outerloop;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("count end: {count}")
}

fn plus_one(mut counter: i32) -> i32 {
    println!("Your given value:{counter}");
    loop {
        counter += 1;
        println!("counter value: {counter}");
        if counter >= 10 {
            println!("cannot calculte values for more than 10");
            break counter * 2;
        }
    }
}

fn loop_it_infinitly() {
    loop {
        println!("hahaaha");
    }
}

fn continue_condition(cond: bool) {
    loop {
        println!("ok");
        if cond {
            continue;
        }
        println!("nahhhhhhhhhhhhh")
    }
}
