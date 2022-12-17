use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut x = 1;
       
    loop {
        let mut guess = String::new();

        println!("Enter a guess: ");
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < secret_number {
            println!("Number is less than goal");
        } else if guess > secret_number {
            println!("Number is greater than goal");
        } else if guess == secret_number {
            break;
        }
        x = x + 1;
    }

    println!("You found the secret number: {secret_number} in {x} tries");
}
