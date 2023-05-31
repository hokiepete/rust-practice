fn main() {
    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);

    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed!");
}

/* YOUR CODE GOES HERE */
fn celsius_to_fahrenheit(celsius_temp: f64) -> f64 {
    (celsius_temp * 9.0 / 5.0) + 32.0
}