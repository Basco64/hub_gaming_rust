use rand::Rng;
use std::cmp::Ordering;
use std::error::Error;

use crate::{utils, User};
use crate::db::DbManager as DB;

pub const GAME_NAME: &str = "guessing";

pub fn play_game(user: &User) -> Result<(), Box<dyn Error>> {
  let user_id = user.id;
  let username = user.username.clone();
  println!("Welcome on the Guessing Game !\n");

    loop {
        utils::clear_terminal();
        let mut remaining_trials: i32 = 10;
        let mut numbers_tested: Vec<u32> = Vec::new();
        let secret_number: u32 = generate_random_number();
        println!("You have {} trials to guess the number between 1 and 100.", remaining_trials);
        println!("Once you exceed 10 attempts, your score will be 0.");
        // println!("The number is: {}", secret_number);
        loop {
            println!("Please enter your guess (1-100):");
            let input: u32 = get_valid_number();
            if !(1..=100).contains(&input) {
                println!("Invalid input. Please enter a number between 1 and 100.");
                continue;
            }

            match input.cmp(&secret_number) {
                Ordering::Less => {
                    println!("Your guess is too low!");
                }
                Ordering::Greater => {
                    println!("Your guess is too high!");
                }
                Ordering::Equal => {
                    println!("Congratulations {}! You guessed the number!", username);
                    break;
                }
            }
            
            remaining_trials -= 1;
            numbers_tested.push(input);
            display_number_tested(&numbers_tested);
        }
        
        let score: i32 = if remaining_trials > 0 {
            remaining_trials * 10
        } else {
            0
        };
        
        println!("Your score is: {}", score);
        match DB::add_game_score(GAME_NAME, user_id, &username, score) {
            Ok(_) => {}
            Err(e) => println!("Failed to save score: {}", e),
        }
        
        match utils::play_again() {
            true => utils::clear_terminal(),
            false => {
                utils::clear_terminal();
                break;
            }
        }
    }
  
    Ok(())
}

fn generate_random_number() -> u32 {
  let mut rng = rand::rng();
  rng.random_range(1..=100)
}

fn get_valid_number() -> u32 {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        
        match input.trim().parse() {
            Ok(amount) => return amount,
            Err(_) => println!("Invalid input. Please enter a valid number:"),
        }
    }
}

fn display_number_tested(array: &[u32]) {
    println!("\nNumber tested : ");
    for (i, number) in array.iter().enumerate() {
        if i == array.len() - 1 {
            print!("{}", number);
        } else {
            print!("{}, ", number);
        }
    }
    println!();
}