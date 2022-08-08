use std::io;

fn main() {
    println!("Welcome to the Gussing Number Game!");
    println!("--- --- --- ---- ---- ---- ---- ---");
    println!("Guess a number beetween 1 to 100: ");
    
    let mut gussed_number = String::new();
    
    io::stdin()
    .read_line(&mut gussed_number)
    .expect("Failed to read the input.");
    println!("--- --- --- ---- ---- ---- ---- ---");

    println!("Your gussed number is: {gussed_number}");

}
