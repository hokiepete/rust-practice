use std::env;
use std::fs;
use std::collections::HashMap;
 
fn main() {
 
    let mut counter: HashMap<&str, u32> = HashMap::new();
   
    let filename = match env::args().nth(1) {
        Some(x) => x,
        None => {
            println!("Must provide filename.");
            std::process::exit(1);
        }
    };
 
    let contents = match fs::read_to_string(&filename) {
        Ok(x) => x.to_lowercase(),
        Err(_) => {
            println!("Unable to read file '{}'.", filename);
            std::process::exit(1);
        }
    };
   
    for token in contents.split_whitespace() {
        let count = counter.entry(token).or_insert(0);
        *count += 1;
    }
   
    let mut largest_count: u32 = 0;
    for (_, value) in counter.iter() {
        if *value > largest_count {
            largest_count = *value;
        }
    }
 
    let mut vec: Vec<&str> = Vec::new();
    for (key, value) in counter.iter() {
        if *value == largest_count {
            vec.push(key)
        }
    }
 
    println!("The most common word(s) are {:?} with a count of {}", vec, largest_count);
}