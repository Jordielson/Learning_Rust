

pub fn check_variable_scope() {
    let s = "hello";

    {
        let s = "world";
        println!("The value to 's' in scope is: {s}");
    }
    println!("The value to 's' out scope is: {s}");
}

pub fn use_string_type() {
    let mut s = String::from("hello");

    s.push_str(", world!");
    
    println!("The value to 's' is: {s}");
}

pub fn interacting_data_move() {
    let s1 = String::from("hello");
    let s2 = s1; // Rust invalid s1

    // println!("{}, world!", s1); // it won’t work
    println!("{}, world!", s2);
}

pub fn interacting_data_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

pub fn pass_value_to_function() {
    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;
    
    makes_copy(x);
}

pub fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

pub fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

pub fn return_value_of_function() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s2 = takes_and_gives_back(s2);

    println!("s1: {s1}, s2: {s2}")
}

pub fn takes_and_gives_back(some_string: String) -> String {
    some_string
}

pub fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

