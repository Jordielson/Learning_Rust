pub fn converte_type() {
    let guess: u32 = "42d".parse().expect("Not a number!");
    println!("Value of guess: {guess}")
}

pub fn create_tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // let (x, y, z) = tup;

    let x = tup.0;
    let y = tup.1;
    let z = tup.2;

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
}

pub fn create_array() {
    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    println!("First month of year: {}", months[0]);
}