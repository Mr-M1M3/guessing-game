use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main(){
    println!("Welcome, to guessing game!!! \n Guess a number from 1 to 100");
    // generates random number
    let random: i32 = rand::thread_rng().gen_range(1..=100);
    loop {
        // empty string to store user input
        let mut guess: String = String::new();
        // reads input from user
        io::stdin().read_line(&mut guess).expect("Get a stable system man...");
        // converts user input to i32
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("I'm pretty sure that I said to guess a number not a string");
                continue;
            }
        };

        // if user types number out of range
        if guess > 100 {
           println!("Didn't I ask to guess a number between 1 & 100?");
           continue;
        }else if guess < 1 {
            println!("Didn't I ask to guess a number between 1 & 100?");
            continue;
        }

        // compares user input to the random number
        match guess.cmp(&random) {
            Ordering::Less => println!("Not too small"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Not too big bro..")
        }
    }
}
