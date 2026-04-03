// String is a growable Buffer UTF-8 encoded

fn main() {
    // func_1();
    // func_2();
    // func_3();
    // func_4();
    // now_try_it_for_string();
    // use_it();
    use_it_2();
}


//
// References => references are also used to  point to a value unlike a pointer which completely changes the ownership when an other variable points to a same value.
fn use_it_2() {
    let s1 = String::from("Hello hi");
    println!("s1 value: {s1}");
    let (s2, len) = reference_func(&s1);
    println!("s1 value still after giving it to s2: {s1}");
    println!("s2 value: {s2}");
    println!(
        "length value of s1: {len} because the reference function give the length of referenced variable."
    );
}

fn reference_func(s: &String) -> (&String, usize) {
    let len = s.len();
    (s, len)
}
//

// fn return multiple values

fn use_it() {
    let s1 = String::from("hello hi");
    let (s2, length_of_string) = return_multiple_values(s1);
    println!(
        "the string is: '{}' \n and length is: {}",
        s2, length_of_string
    )
}

fn return_multiple_values(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

//

//

fn now_try_it_for_string() {
    let s1 = give_up_ownership();
    println!("s1 from give_up: {s1}");
    let s2 = String::from("ooooohhhhh");
    println!("s2 before give_and_take: {s2}");
    let s3 = take_and_give_back_ownership(s2);
    println!("s2 after take_and_give is gone to s3");
    println!("s3 after take_and_give: {s3}");
}

fn take_and_give_back_ownership(take_the_string: String) -> String {
    take_the_string
}

fn give_up_ownership() -> String {
    let some_string = String::from("HI");
    some_string
}
//

fn func_4() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}

fn func_3() {
    //If we do want to deeply copy the heap data of the String , not just the stack data, we can use a common method called clone

    let s1 = String::from("hello");
    println!("s1: {s1}");
    let s2 = s1.clone();
    println!("s1: {s1}");
    println!("s2: {s2}")
}

fn func_2() {
    let s1 = String::from("Hello");
    println!("s1: {s1}");
    let s2 = s1;
    // println!("s1: {s1}");
    println!("s2: {s2}")
}

fn func_1() {
    let mut s = String::from("Hello"); // its a heap
    println!("{s}"); // before
    s.push_str(", World"); // appends a literal to a String
    println!("{s}") // after
}
