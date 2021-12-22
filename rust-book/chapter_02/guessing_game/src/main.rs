use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("{}","Guessing Game!".on_blue().italic().bold());
    println!("{}","Please input your guess".yellow());

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is: {}", secret_number);
    loop {
        /* Get input in terminal from the user. 
        "mut" allow this file to be modified */
        let mut guess = String::new();

        io::stdin()
            // guess should be 'mut' to get a value
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        // Convert "guess" to a "u32" number type
        // "trim()" will eliminate any whitespace at the beginning and end
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            },
        };
    
        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small, try again!".red().italic()),
            Ordering::Greater => println!("{}","Too big, try again!".red().italic()),
            Ordering::Equal => {
                println!("{}","You win!".green().bold());
                break;
            }
        }
    }
}
