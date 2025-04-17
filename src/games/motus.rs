use reqwest::blocking;
use std::error::Error;
use rand::Rng;

use crate::db::DbManager as DB;
use crate::User;
use crate::utils;

pub const GAME_NAME: &str = "motus";

#[derive(serde::Deserialize)]
struct ApiWord {
    name: String,
    #[serde(default)] // Rend le champ optionnel
    _categorie: Option<String>,
}

enum Theme {
    English,
    FrAgriculture,
    FrArmee,
    FrAnimaux,
    FrIndustrie,
    FrNourriture,
}

impl Theme {
    fn from_choice(choice: u32) -> Option<Self> {
        match choice {
            1 => Some(Theme::English),
            2 => Some(Theme::FrAgriculture),
            3 => Some(Theme::FrArmee),
            4 => Some(Theme::FrAnimaux),
            5 => Some(Theme::FrIndustrie),
            6 => Some(Theme::FrNourriture),
            _ => None,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            Theme::English => "english words",
            Theme::FrAgriculture => "french agriculture",
            Theme::FrArmee => "french army",
            Theme::FrAnimaux => "french animals",
            Theme::FrIndustrie => "french industry",
            Theme::FrNourriture => "french food",
        }
    }

    fn api_endpoint(&self) -> &'static str {
        match self {
            Theme::English => "https://random-word-api.herokuapp.com/word?number=50&length=5",
            Theme::FrAgriculture => "https://trouve-mot.fr/api/categorie/11/50",
            Theme::FrArmee => "https://trouve-mot.fr/api/categorie/26/50",
            Theme::FrAnimaux => "https://trouve-mot.fr/api/categorie/19/50",
            Theme::FrIndustrie => "https://trouve-mot.fr/api/categorie/9/50",
            Theme::FrNourriture => "https://trouve-mot.fr/api/categorie/5/50",
        }
    }
}

pub fn play_game(user: &User) -> Result<(), Box<dyn Error>> {
    let user_id = user.id;
    let username = user.username.clone();

    utils::clear_terminal();
    println!("Welcome on the Motus Game !\n");

    let theme = match select_theme() {
        Some(theme) => theme,
        None => {
            println!("Invalid choice. Exiting game.");
            return Ok(());
        }
    };

    println!("You selected the theme: {}", theme.as_str());

    let words = fetch_words_from_api(theme.api_endpoint())?;
    if words.is_empty() {
        println!("No words found for the selected theme. Exiting game.");
        return Ok(());
    }
        
    loop {
        let mut attempts = 10;
        let mut rng = rand::rng();
        let secret_word = &words[rng.random_range(0..words.len())];
        let secret_chars: Vec<char> = secret_word.chars().collect();
        
        let mut correct_chars: Vec<char> = vec![' '; secret_word.len()];
        let mut misplaced_chars: Vec<char>;
        
        //  println!("{} is the secret word!", secret_word);
        println!("The secret word contains {} letters.", secret_word.len());
        loop {
            let input = utils::get_valid_input("Please enter your guess: ");
            let input = utils::remove_accents(input.trim());

            if !input.len().eq(&secret_word.len()) && attempts > 0 {
                    attempts -= 1;
                    println!("Word length mismatch. You have {} attempts left.", attempts);
                    continue;
            }
            let users_chars: Vec<char> = input.trim().chars().collect();
            misplaced_chars = Vec::new();

            for (index, &user_char) in users_chars.iter().enumerate() {
                if let Some(&secret_char) = secret_chars.get(index) {
                    if secret_char == user_char {
                        correct_chars[index] = user_char;
                    } else {
                        correct_chars[index] = ' ';
                        if secret_chars.contains(&user_char) {
                            misplaced_chars.push(user_char);
                        }
                    }
                } else {
                    println!("Index out of bounds: {}", index);
                    break;
                }
            }
            if correct_chars.iter().all(|&c| c != ' ') {
                println!("Congrats! You guessed the word: {}", secret_word);
                let score = attempts * 10;
                match DB::add_game_score(GAME_NAME, user_id, &username, score) {
                    Ok(_) => {},
                    Err(e) => println!("Failed to save score: {}", e),
                }
                break;
            }

            if attempts > 0 {
                attempts -= 1;
            }

            println!("Correct letters in the right position: {:?}", correct_chars);
            println!("Misplaced letters: {:?}", misplaced_chars);
            println!("You have {} attempts left.", attempts);
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

fn select_theme() -> Option<Theme> {
    println!("Select a theme: ");
    println!("1. English");
    println!("2. French Agriculture");
    println!("3. French Army");
    println!("4. French Animals");
    println!("5. French Industry");
    println!("6. French Food");

    let choice = utils::get_valid_choice();
    Theme::from_choice(choice)
}

fn fetch_words_from_api(api_url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let response = blocking::get(api_url)?;
    if !response.status().is_success() {
        return Err(format!("Failed to fetch words: HTTP {}", response.status()).into());
    }

    if api_url.contains("random-word-api") {
        let words: Vec<String> = response.json()?;
        println!("Fetched {} words from API.", words.len());
        Ok(words)
    } else {
        let api_words: Vec<ApiWord> = response.json()?;
        let words: Vec<String> = api_words
            .into_iter()
            .map(|word| utils::remove_accents(&word.name))
            .collect();
        println!("Fetched {} words from API.", words.len());
        Ok(words)
    }
}