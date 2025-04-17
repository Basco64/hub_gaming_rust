mod db;
mod utils;
mod games {
    pub mod guessing;
    pub mod motus;
    pub mod reflex;
}

use db::DbManager as DB;
use utils::*;
use games::guessing;
use games::motus;
use games::reflex;

struct User{
    id: i64,
    username: String,
}


fn main() -> Result<(), Box<dyn std::error::Error>>{
    DB::init()?;
    utils::clear_terminal();
    println!("Welcome to the Gaming Hub!");
    let username = get_valid_input("Please enter your username: ");
    let user_session = get_or_create_user(&username)?;
    loop {
        println!("Main menu\n");
        println!("1. Play a game");
        println!("2. Your best scores");
        println!("3. Leaderboards");
        println!("4. Quit");

        let choice = get_valid_choice();
        utils::clear_terminal();
        match choice {
            1 => {
                println!("Choose a game:");
                println!("1. Guessing Game");
                println!("2. Motus");
                println!("3. Reflex");
                println!("4. Quit");

                let game_choice = get_valid_choice();
                match game_choice {
                    1 => {
                        if let Err(e) = guessing::play_game(&user_session) {
                            println!("Error playing Guessing Game: {}", e);
                        }
                    },
                    2 => {
                        if let Err(e) = motus::play_game(&user_session) {
                            println!("Error playing Motus: {}", e);
                        }
                    },
                    3 => {
                        if let Err(e) = reflex::play_game(&user_session) {
                            println!("Error playing Reflex: {}", e);
                        }
                    },
                    4 => {
                        println!("Thanks for playing, {}! See you soon!", user_session.username);
                        return Ok(());
                    },
                    _ => println!("Invalid choice, please try again."),
                }
            },
            2 => {
                if let Err(e) = get_user_best_scores(&user_session) {
                    println!("Error retrieving scores: {}", e);
                }
            },
            3 => {
                if let Err(e) = display_all_leaderboards() {
                    println!("Error displaying leaderboards: {}", e);
                }
            },
            4 => break,
            _ => println!("Invalid choice, please try again."),
        }
    }
    println!("Thanks for playing, {}! See you soon!", user_session.username);
    Ok(())
}

fn get_or_create_user(username: &str) -> Result<User, Box<dyn std::error::Error>> {
    if let Some((id, name)) = DB::get_user_by_username(username)? {
        println!("It's a pleasure to see you again, {}!", name);
        return Ok(User {
            id,
            username: name,
        });
    }
    
    let id = DB::create_user(username)?;
    
    Ok(User {
        id,
        username: username.to_string(),
    })
}

fn display_all_leaderboards() -> Result<(), Box<dyn std::error::Error>> {
    let games = vec!["guessing", "motus", "reflex"];
    
    for game in games {
        println!("Leaderboard for {}:", game);
        match DB::get_leaderboard(game) {
            Ok(leaderboard) => {
                if leaderboard.is_empty() {
                    println!("  No scores recorded yet.");
                } else {
                    for (i, (name, score)) in leaderboard.iter().enumerate().take(5) {
                        println!("  {}. {}: {}", i + 1, name, score);
                    }
                }
            },
            Err(e) => println!("  Could not load leaderboard: {}", e),
        }
    }
    
    Ok(())
}

fn get_user_best_scores(user: &User) -> Result<(), Box<dyn std::error::Error>> {
    let user_id = user.id;
    let username = &user.username;
    
    println!("\nBest scores for {}:", username);
    
    match DB::get_user_best_scores(user_id) {
        Ok(best_scores) => {
            for (game, score) in best_scores {
                match score {
                    Some(score_value) => println!("\t{}: {}", game, score_value),
                    None => println!("\t{}: No score yet", game),
                }
            }
        },
        Err(e) => println!("Error retrieving scores: {}", e),
    }
    
    println!();
    Ok(())
}
