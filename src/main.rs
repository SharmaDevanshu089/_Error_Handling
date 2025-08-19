use std::io;
fn main() {
    let mut input = String::new();
    println!("Enter a number:");

    io::stdin().read_line(&mut input).unwrap();

    // Try to parse input into i32
    let num = input.trim().parse::<i32>();  

    // ❓ Your task: handle Result here with match
    // If Ok(n) → print "You entered n"
    // If Err(e) → print "Invalid input!"
    match num {
        Ok(n) => print!("The Output is {}", n),
        Err(n) => println!("Fatal Error Occured. Probably Cuz u entered something that should be number but is not")
    }
}