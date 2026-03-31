// loops ==> loop, while, for               3

fn main() {
    //
    // loop_it_infinitly()
    // continue_condition(false)
    let value = plus_one(1);
    println!("from MAIN FUNCTION: {value}");
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
