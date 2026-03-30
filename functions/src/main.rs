fn main() {
    another_function();
    param_func(45);
}

fn another_function() {
    println!("ok, now I am from another function.");
}

fn param_func(x: i32) {
    println!("The value  of x is {x}")
}
