fn main() {
    // another_function();
    // param_func(45);
    // print_label_measurements(5, 'h');
    // error_due_to_statement_as_statement_do_not_return_value();
    // error_due_to_statement_as_statement_do_not_return_value_2222();
    // new_scope_block();
    // let x = five(); // here, the value that returns (comes) from the five() functions (that is 5 here) bound to x, so x can be printed
    // println!("value of x: {x}");
    // let x = five_return();
    // println!("value of x: {x}");
    // let x = func_param__plus_one(4);
    // println!("{x}");

    conditional(false);
    conditional(true);
}

fn conditional(condition: bool) {
    let number = if condition { 5 } else { 6 };
    println!("value of number: {number}")
}

fn func_param__plus_one(x: i32) -> i32 {
    x + 1
}

fn five_return() -> i32 {
    return 5;
}

fn five() -> i32 {
    5 // last expression is the return value implicitly. until we specified a return value
}

fn new_scope_block() {
    let y = {
        let x = 5;
        x + 1
    };
    println!("value of y: {y}",)
}

fn another_function() {
    println!("ok, now I am from another function.");
}

fn param_func(x: i32) {
    println!("The value  of x is {x}")
}

fn print_label_measurements(value: i32, label: char) {
    println!("The value is {value} and label is: {label}");
    println!("complete: {value}{label}");
}

// fn error_due_to_statement_as_statement_do_not_return_value(){
//     let x = (let y = 6);
//     println!("value of x: {x}");
// }

// fn error_due_to_statement_as_statement_do_not_return_value_2222(){
//     let x = let y = 6;
//     println!("value of x: {x}");
// }
