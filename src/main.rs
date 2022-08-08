use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the Gussing Number Game!");
    println!("--- --- --- ---- ---- ---- ---- ---");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess a number beetween 1 to 100: ");

        let mut input_str = String::new();

        io::stdin()
            .read_line(&mut input_str)
            .expect("Failed to read the input.");
        let guessed_number: u32 = input_str
            .trim()
            .parse()
            .expect("Input needs to be a number!");

        match guessed_number.cmp(&secret_number) {
            Ordering::Less => println!("It's smaller than the secret number!"),
            Ordering::Greater => println!("It's bigger than the secret number!"),
            Ordering::Equal => {
                println!("*** You won the game! ***");
                break;
            }
        }

        println!("---- ---- ---- ---- ---- ---- ---- ----");
    }
}
