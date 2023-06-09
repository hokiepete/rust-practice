use std::io;
use rand::Rng; // 0.8.5
 
fn main() {
    // Generate random number in the range [0, 99]
    let num = rand::thread_rng().gen_range(1..101);
    // initialize result
    let mut result: i32 = -1;
    // loop while guess is incorrect
    loop {
        loop {
            // initialize input text
            let mut input_text = String::new();
            println!("Please guess a number between 1 and 100: ");
            io::stdin().read_line(&mut input_text);
            let input_result = input_text.trim().parse::<i32>();
            match input_result {
                Ok(message) => {
                    result = message;
                    break
                }
                Err(_) => {
                   println!("Input must be a number. Letters and special characters are not allowed")
                }
            }
        }
        if result > num {
            println!("Your guess of {} was too high", result);
        } else if result < num {
            println!("Your guess of {} was too low", result);
        } else {
            break;
        }
    }
    println!("Congratulations your guess of {} was correct, you win!", result);
}