use std::fmt;
 
struct Satellite {
    name: String,
    velocity: f64 // miles per second
}
 
/* YOUR CODE GOES HERE */
impl fmt::Display for Satellite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} going at {} miles per second", self.name, self.velocity)
    }
}
 
fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };
    println!("hubble is {}", hubble);
}