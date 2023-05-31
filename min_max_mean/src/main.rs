fn main() {
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32 = -9999;
    let mut min: i32 = 9999;
    let mut mean: f64 = 0.0;

    /* YOUR CODE GOES HERE */
    for number in numbers {
        if number > max {
            max = number;
        }
        if number < min {
            min = number;
        }
        mean += number as f64;
    }
    mean = mean / numbers.len() as f64;
    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed!");
}
