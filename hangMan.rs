use std::io::{self, Write};

pub struct Game {
    guessed: Vec<i32>,
    score: i32,
    isPlaying: bool,
    word: String,
}

impl Game {
    pub fn new() -> Self {
        Self {
            guessed: vec![0; 26],
            score: 0,
            isPlaying: true,
            word: String::new(),
        }
    }

    pub fn play(&mut self) {
        while (self.word.is_empty()){
            println!("Enter your secret word: ");
            io::stdout().flush().expect("Failed to flush stdout");
            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("Failed to read input");
            self.word = line.trim().to_string().to_lowercase();
            if !(self.word.chars().all(|c| c.is_alphabetic())){
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                println!("Please enter an alphabetic word.");
                self.word = String::new();
                continue;
            } else {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            }

        }

        println!("Beginning game.......");


        while self.isPlaying {
            self.printCurrentGameState();
            if (self.score == 6){
                println!("Oh no! You lost!.");
                println!("The word was: {}", self.word);
                self.isPlaying = false;
                return;
            }
            println!("\nEnter your guess (a single letter): ");
            io::stdout().flush().unwrap();

            let mut guess = String::new();
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read input");
            let guess = guess.trim().to_lowercase();
            if guess.len() != 1 {
                println!("Please enter a single lowercase letter.");
                continue;
            }
            if (!guess.chars().all(|c| c.is_alphabetic())){
                println!("Please enter alphabetic string.");
                continue;
            }
            let guessBytes = guess.as_bytes();
            let index = (guessBytes[0] - b'a') as usize;

            if self.guessed[index] != 0 {
                println!("Already guessed!: {}", guessBytes[0] as char);
                continue;
            }
            if self.word.contains(&(guessBytes[0] as char).to_string()) {
                self.guessed[index] = 2;
            } else {
                self.guessed[index] = 1;
                self.score += 1;
            }
            if self.word.chars().all(|c| self.guessed[(c as u8 - b'a') as usize] == 2) {
                println!("Congrats! You won! The word was {}", self.word);
                self.printCurrentGameState();
                self.isPlaying = false;
            }

        }
    }

    fn printCurrentGameState(&self) {
        let wordBytes = self.word.as_bytes();
        println!("\n");
        for &j in wordBytes {
            if self.guessed[(j - b'a') as usize] == 2 {
                print!("{} ", j as char);
            } else {
                print!("_ ");
            }
        }
        println!("\nScore: {}", self.score);
        self.printHangMan(self.score);
    }
    fn printHangMan(&self, score: i32){
        let stages = [
        r"
            -----
            |    
            |    
            |    
            |    
           _|_",
                    r"
            -----
            |   O
            |    
            |    
            |    
           _|_",
                    r"
            -----
            |   O
            |   |
            |    
            |    
           _|_",
                    r"
            -----
            |   O
            |  /|
            |    
            |    
           _|_",
                    r"
            -----
            |   O
            |  /|\
            |    
            |    
           _|_",
                    r"
            -----
            |   O
            |  /|\
            |  / 
            |    
           _|_",
                    r"
            -----
            |   O
            |  /|\
            |  / \
            |    
           _|_",
                ];
            let index = std::cmp::min(score as usize, stages.len() - 1);
            println!("{}", stages[index]);
    }
}

fn main() {
    let mut game = Game::new();
    game.play();
}
