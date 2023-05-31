use std::io;
use rand::Rng; // 0.8.5
 
fn main() {
    // Generate random number in the range [1, 100]
    let num = rand::thread_rng().gen_range(1..101);
    // initialize run flag
    let mut run: bool = true;
    // initialize result
    let mut result: i32 = -1;
    // loop while guess is incorrect
    while run {
        // initialize input text
        let mut input_text = String::new();
        println!("Please guess a number between 1 and 100: ");
        io::stdin().read_line(&mut input_text);
        result = input_text.trim().parse::<i32>().unwrap();
        if result > num {
            println!("Your guess of {} was too high", result);
        } else if result < num {
            println!("Your guess of {} was too low", result);
        } else {
            run = false;
        }
    }
    println!("Congratulations your guess of {} was correct, you win!", result);
}