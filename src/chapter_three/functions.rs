pub fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

pub fn learn_statements_and_expressions() -> () {
    let y: i32 = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

pub fn return_five() -> i32 {
    5
}