fn main() {
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");
   
    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");
   
    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");  
    
    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");
   
    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");
   
    let test6 = "";
    assert_eq!(trim_spaces(test6), "");
   
    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(test7), "🚀");
    println!("Tests passed!");
}
 
/* YOUR CODE GOES HERE */
fn trim_spaces(string: &str) -> &str {
    // forward pass, trim beginning
    let mut front_idx: usize = 0;
    for (i, c) in string.chars().enumerate(){
        front_idx = i;
        if c != ' ' {
            break;
        }
    }
    // backward pass, trim ending
    let mut back_idx: usize = 0;
    for (i, c) in string.chars().rev().enumerate(){
        back_idx = i;
        if c != ' ' {
            break;
        }
    }
 
    // check that it's not just an empty string
    let end_idx: usize = string.len()-back_idx;
    let mut result: &str = "";
    if end_idx > front_idx{
        result = &string[front_idx..end_idx];
    }
    result
   
}