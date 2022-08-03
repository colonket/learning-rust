use rand::Rng;
use std::cmp::Ordering;
use std::io;

/// .
///
/// # Panics
///
/// Panics if .
pub fn main() {
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1,101);

    loop {
        println!("Please input guess ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}