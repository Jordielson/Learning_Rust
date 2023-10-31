pub fn learn_constants() {
    const THREE_HOURS_IN_SECONDS: i16 = SEGUNDS_IN_MINUTE * MINUTES_IN_HOUR * HOURS;
    const SEGUNDS_IN_MINUTE: i16 = 60;
    const MINUTES_IN_HOUR: i16 = 60;
    const HOURS: i16 = 3;
    println!("Three hours in seconds {THREE_HOURS_IN_SECONDS}");
}