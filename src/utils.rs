pub fn clear_terminal() {
    // Pour Windows
    if cfg!(target_os = "windows") {
        let _ = std::process::Command::new("cmd")
            .args(["/c", "cls"])
            .status();
    } else {
        // Pour Unix 
        let _ = std::process::Command::new("clear")
            .status();
    }
}

pub fn get_valid_choice() -> u32 {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        
        match input.trim().parse() {
            Ok(choice) => return choice,
            Err(_) => {
                println!("Invalid input. Please enter a number:");
                continue;
            }
        }
    }
}

pub fn get_valid_input(prompt: &str) -> String {
    use std::io::Write;
    loop {
        println!("{}", prompt);
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let input = input.trim();
        if !input.is_empty() {
            return input.to_string();
        } else {
            println!("Input cannot be empty. Please try again:");
        }
    }
}

pub fn play_again() -> bool {
    loop {
        let input = get_valid_input("Do you want to play again? (y/n)");
        match input.to_lowercase().as_str() {
            "y" => return true,
            "n" => return false,
            _ => println!("Invalid input. Please enter 'y' or 'n':"),
        }
    }
}


pub fn remove_accents(input: &str) -> String {
    use unidecode::unidecode;
    
    unidecode(&input.to_lowercase())
}