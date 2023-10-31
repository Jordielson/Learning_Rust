pub fn learn_variables() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

pub fn learn_constants() {
    const THREE_HOURS_IN_SECONDS: i16 = SEGUNDS_IN_MINUTE * MINUTES_IN_HOUR * HOURS;
    const SEGUNDS_IN_MINUTE: i16 = 60;
    const MINUTES_IN_HOUR: i16 = 60;
    const HOURS: i16 = 3;
    println!("Three hours in seconds {THREE_HOURS_IN_SECONDS}");
}

pub fn shadowing_variables() {
    let x: i8 = 5;

    let x: i8 = x + 1;

    {
        let x: i8 = x * 20;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

pub fn change_variable_type_with_shadowing() {
    let spaces: &str = "   ";
    println!("Type of spaces: {}", type_of(spaces));
    let spaces: usize = spaces.len();
    println!("Type of spaces: {}", type_of(spaces));

}

fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}