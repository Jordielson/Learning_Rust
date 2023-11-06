pub fn check_number_is_divisible() {
    let number: u32 = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}


pub fn use_contidional_in_statement() -> () {
    let contition = true;
    let number: i32 = if contition { 5 } else { 6 };

    println!("The value of number is {number}");
}

pub fn return_values_from_loop() -> () {
    let mut counter:u32 = 0;

    let result:u32 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

pub fn define_loop_labels() -> () {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

pub fn reverse_loop_range() -> (){
    for (index, number) in (1..4).rev().enumerate() {
        println!("Element in index {index} is {number}!");
    }
    println!("LIFTOFF!!!");
}