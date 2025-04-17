use std::error::Error;
use rand::Rng;
use std::time::Instant;

use crate::db::DbManager as DB;
use crate::User;
use crate::utils;

pub const GAME_NAME: &str = "reflex";

enum GamePhase{
    Phase1,
    Phase2,
}
struct Game {
    words_phase1: Vec<&'static str>,
    words_phase2: Vec<&'static str>,
    score:i32,
    total_reaction_time : u64,
    phase1_reaction_times: Vec<u128>,
    target_reaction_time : u128
}

impl Game {
    fn new() -> Game {
        Game {
            words_phase1: vec!["left", "right"],
            words_phase2: vec!["left", "right", "LEFT", "RIGHT"],
            score: 0,
            total_reaction_time: 0,
            phase1_reaction_times: Vec::new(),
            target_reaction_time: 0,
        }
    }

    fn run(&mut self, phase: &GamePhase){
        self.display_instruction(phase);
        self.run_phase(phase);
    }

    fn display_instruction(&mut self, phase: &GamePhase) {
        match phase {
            GamePhase::Phase1 => {
                println!("Press the 'q' or 'd' key as quickly as possible based respectively on the words 'left' or 'right' displayed.");
                println!("Then, press 'Enter' to validate your choice.");
            }
            GamePhase::Phase2 => {

                self.target_reaction_time = self.phase1_reaction_times.iter().sum::<u128>() / self.phase1_reaction_times.len() as u128;
                self.target_reaction_time += self.target_reaction_time / 2;
                println!("Now, if the words LEFT and RIGHT are in uppercase, you must press the opposite key.");
                println!("As before, you must press 'Enter' to validate your choice.");
                println!("Based on your scores from Phase 1, you must respond in less than {} ms to earn points.", self.target_reaction_time);

            }
        }
        wait_for_enter();
    }

    fn run_phase(&mut self, phase: &GamePhase) {
        let mut rng = rand::rng();
        match phase {
            GamePhase::Phase1 => {
                for _ in 0..10 {
                    let randow_word = self.words_phase1[rng.random_range(0..self.words_phase1.len())];
                    let reaction_time = self.display_word(randow_word, phase);
                    self.total_reaction_time += reaction_time;
                    wait_for_enter();
                }
            }
            GamePhase::Phase2 => {
                for _ in 0..10 {
                    let randow_word = self.words_phase2[rng.random_range(0..self.words_phase2.len())];
                    let reaction_time = self.display_word(randow_word, phase);
                    self.total_reaction_time += reaction_time;
                    wait_for_enter();
                }
            }
        }
    }

    fn display_word(&mut self, word: &str, phase: &GamePhase) -> u64{
        let start_time = Instant::now();
        utils::clear_terminal();
        println!("{}", word);
        let key = get_key_pressed();
        let reaction_time = start_time.elapsed().as_millis();

        if self.is_correct_key(word, key){
            match phase {
                GamePhase::Phase1 => {
                    println!("Correct! Reaction time: {} ms", reaction_time);
                    self.score += 1;
                    self.phase1_reaction_times.push(reaction_time);
                }
                GamePhase::Phase2 => {
                    if reaction_time <= self.target_reaction_time {
                        println!("Correct! Reaction time: {} ms", reaction_time);
                        self.score += 1;
                    } else {
                        println!("Correct but too slow! Reaction time: {} ms", reaction_time);
                    }
                }
            }
        } else {
            println!("Wrong key!");
        }

        reaction_time as u64

    }

    fn is_correct_key(&self, word: &str, key: char) -> bool {
        match key {
            'd' if word == "right" => true,
            'q' if word == "left" => true,
            'd' if word == "LEFT" => true,
            'q' if word == "RIGHT" => true,
            _ => false,
        }
    }
}

fn wait_for_enter() {
    use std::io::Write;
    loop {
        println!("Press 'Enter' to continue...");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let input = input.trim();
        if !input.is_empty() {
            continue;
        } 
        break;
    }
}

fn get_key_pressed() -> char {
    let input = utils::get_valid_input("Press a key:");
    input.chars().next().unwrap_or('\n')
}

pub fn play_game(user: &User) -> Result<(), Box<dyn Error>> {
    let user_id = user.id;
    let username = user.username.clone();
    let mut game = Game::new();
    println!("Welcome on the Reflex Game !\n");

    loop{
        game.run(&GamePhase::Phase1);
        utils::clear_terminal();
        game.run(&GamePhase::Phase2);

        match DB::add_game_score(GAME_NAME, user_id, &username, game.score) {
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