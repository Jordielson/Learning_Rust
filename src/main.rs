use crate::chapter_three::practice;

pub mod chapter_three;

fn main() {
    let n = 5683;
    let result = practice::generate_fibonacci_number(n);
    println!("Fibonacci({}) = {}", n, result);
}
