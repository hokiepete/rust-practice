use std::env;
use std::fs;
 
fn main() {
    if env::args().len() != 3 {
        println!("Must provide 2 arguments filename and person name.");
        return;
    }
   
    let filename = env::args().nth(1).unwrap();
    let name = env::args().nth(2).unwrap();
   
    let contents = fs::read_to_string(&filename).unwrap();
    for line in contents.lines() {
        if name.trim().to_lowercase() == line.trim().to_lowercase(){
            println!("Yes, {} is in file {}", name, filename);
            return;
        }
    }
    println!("Sorry, but {} was not found in file {}", name, filename);
 
}