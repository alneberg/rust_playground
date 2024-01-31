use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let random_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("What's the number? Place your guesses!");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read guess.");
        
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please specify an integer number");
                continue;
            },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
