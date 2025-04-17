# 🎮 Hub Gaming

A collection of word and reaction games built in Rust, featuring **Motus**, **Reflex**, and **Guessing Game**.

---

## ✨ Features

- **Multiple Game Selection**: Choose from different games to play  
- **Motus**: Word-guessing game with feedback on correct letters  
- **Reflex**: Test your reaction time and accuracy  
- **Guessing Game**: Classic number guessing game with hints
- **Theme Selection**: Choose from different themes for the Motus game  
- **Score Tracking**: Save your high scores to a local database  

---

## 📝 Description

**Hub Gaming** is a terminal-based gaming platform that offers various mini-games to challenge your word knowledge and reflexes. The available games include:

### 🎯 Motus

A word-guessing game similar to Wordle where you:

- Select a theme (English or French categories)
- Guess a hidden word within a limited number of attempts
- Receive feedback on correct letters and their positions
- Score points based on remaining attempts

### ⚡ Reflex

A reaction test game where you:

- Respond to visual prompts as quickly as possible
- Follow directional commands (left/right)
- Improve your reaction time through two challenging phases

### 🔢 Guessing Game

A classic number guessing game where you:

- Try to guess a secret number between 1 and 100
- Receive hints whether your guess is too high or too low
- Challenge yourself to find the number in as few attempts as possible
- Earn higher scores by guessing correctly with fewer attempts

---

## 🛠️ Technical Details

- Built with **Rust**
- Uses **SQLite** for score tracking
- Integrates with external APIs to fetch word dictionaries
- Handles accented characters through Unicode normalization
- Employs error handling for robust performance

---

## 🚀 Installation

### Pre-built Binaries

Download the appropriate binary for your operating system:

- Windows  
- Linux  
- macOS

### Build from Source

Make sure you have Rust installed, then run:

```bash
git clone git clone https://github.com/Basco64/hub_gaming_rust.git
cd hub_gaming_rust
cargo build --release