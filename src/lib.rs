extern crate rand;

use rand::Rng;
use std::io;
use std::fs::File;
use std::fs;
use std::io::prelude::*;

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
		.expect("no input");
    line.trim().to_string()
}

fn read_line_space() -> String {
    println!("");
    let aux = read_line();
    aux
}

struct GameData {
    tries: Vec<char>,
    word: Vec<char>,
    found: Vec<bool>,
    left: usize,
    lives: u32,
}

impl GameData {
    fn new(word: String, lives: u32) -> GameData {
        let word: Vec<char> = word.chars().collect();
        let found = vec![false; word.len()];
        let left = word.len();
        GameData {
            tries: vec![],
            word,
            found,
            left,
            lives,
        }
    }
    fn guess(&mut self, x: char) {
        let mut ok = false;

        for ch in &self.tries {
            if ch == &x {
                println!("Letter {} already searched.", x);
                ok = true;
                break;
            }
        }
        if ok {
            print!("Letters tried: ");
            for ch in &self.tries {
                print!("{} ", ch);
            }
            println!("");
            return;
        }
        self.tries.push(x);
        for i in 0..self.word.len() {
            if self.word[i] == x {
                self.found[i] = true;
                self.left -= 1;
                ok = true;
            }
        }
        if !ok {
            self.lives -= 1;
            println!("Letter not found. {}",
                if self.lives == 0u32 {
                    String::from("You are out of lives!")
                } else if self.lives == 1 {
                    String::from("You have 1 life.")
                } else {
                    format!("You have {} lives.", self.lives)
                }
            );
        } else {
            println!("Letter {} found", x);
        }
    }
    fn print(&self) {
        for i in 0..self.word.len() {
            if self.found[i] {
                print!("{} ", self.word[i]);
            } else {
                print!("_ ");
            }
        }
        println!("");
    }
}

struct Dictionaire {
    words: Vec<String>,
}

impl Dictionaire {
    fn new(file: String) -> Dictionaire {
        //read file with words
        let mut f = File::open(file).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("could not read from file");
        let words: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();

        Dictionaire {
            words,
        }
    }
    fn rand_word(&self) -> String {
        //get rand from words
        let rnd = rand::thread_rng().gen_range(0,self.words.len());

        self.words[rnd].clone()
    }
}

enum GameState {
    Lost(String),
    Won(String),
    Menu,
    Playing(GameData),
    Settings,
}

impl GameState {
    fn print(&self) {
        match self {
            &GameState::Lost(ref word) => println!("You are out of guesses!\nThe answer was {}.", word),
            &GameState::Won(ref word) => println!("You won!\nThe word is {}.", word),
            &GameState::Menu => println!("Start game? [y/n] Settings: [s]"),
            &GameState::Playing(ref data) => data.print(),
            &GameState::Settings => println!("Choose difficulty: [e/m/h] View stats: [s] Reset progress: [r]"),
        }
    }
}

pub struct SaveGame {
    games: (u32, u32),
    easy: (u32, u32),
    medium: (u32, u32),
    hard: (u32, u32),
}

impl SaveGame {
    pub fn new(file: String) -> SaveGame {
        let mut f = File::open(file).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("could not read from file");
        let data: Vec<u32> = contents.trim().split("\n").map(|x| x.to_string().trim().parse::<u32>().unwrap()).collect();

        SaveGame {
            games: (data[0], data[1]),
            easy: (data[2], data[3]),
            medium: (data[4], data[5]),
            hard: (data[6], data[7]),
        }
    }
    fn print(&self) {
        println!("");
        println!("*****************************");
        println!("Games played: {}", self.games.0);
        println!("Games won: {}", self.games.1);
        println!("Easy games played: {}", self.easy.0);
        println!("Easy games won: {}", self.easy.1);
        println!("Medium games played: {}", self.medium.0);
        println!("Medium games won: {}", self.medium.1);
        println!("Hard games played: {}", self.hard.0);
        println!("Hard games won: {}", self.hard.1);
        println!("*****************************");
        println!("");
    }
    fn update(&mut self, end: GameState, diff: &Difficulty) {
        match end {
            GameState::Won(_) => {
                self.games = (self.games.0 + 1, self.games.1 + 1);
                match diff {
                    &Difficulty::Easy => self.easy = (self.easy.0 + 1, self.easy.1 + 1),
                    &Difficulty::Medium => self.medium = (self.medium.0 + 1, self.medium.1 + 1),
                    &Difficulty::Hard => self.hard = (self.hard.0 + 1, self.hard.1 + 1),
                }
            }
            GameState::Lost(_) => {
                self.games = (self.games.0 + 1, self.games.1);
                match diff {
                    &Difficulty::Easy => self.easy = (self.easy.0 + 1, self.easy.1),
                    &Difficulty::Medium => self.medium = (self.medium.0 + 1, self.medium.1),
                    &Difficulty::Hard => self.hard = (self.hard.0 + 1, self.hard.1),
                }
            }
            _ => panic!("Called with wromg argument"),
        }
    }
    fn reset(&mut self) {
        self.games.0 = 0u32;
        self.games.1 = 0u32;
        self.easy.0 = 0u32;
        self.easy.1 = 0u32;
        self.medium.0 = 0u32;
        self.medium.1 = 0u32;
        self.hard.0 = 0u32;
        self.hard.1 = 0u32;
    }
}

impl Drop for SaveGame {
    fn drop(&mut self) {
        fs::remove_file("savegame").unwrap();
        let mut file = File::create("savegame").unwrap();
        let contents = format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n", self.games.0, self.games.1,
                                                                   self.easy.0, self.easy.1,
                                                                   self.medium.0, self.medium.1,
                                                                   self.hard.0, self.hard.1);
        file.write_all(contents.as_bytes()).unwrap();
    }
}

enum Difficulty {
    Easy,
    Medium,
    Hard,
}

pub struct Game {
    words: Dictionaire,
    state: GameState,
    diff: Difficulty,
    save: SaveGame,
}

impl Difficulty {
    fn get_lives(&self) -> u32 {
        match self {
            &Difficulty::Easy => 15u32,
            &Difficulty::Medium => 10u32,
            &Difficulty::Hard => 5u32,
        }
    }
}

impl Game {
    pub fn new(words: String, save: SaveGame) -> Game {
        let words = Dictionaire::new(String::from(words));
        let state = GameState::Menu;

        Game {
            words,
            state,
            diff: Difficulty::Easy,
            save,
        }
    }
    pub fn run(&mut self) {
        loop {
            self.state.print();
            let mut change: Option<GameState> = None;
            match &mut self.state {
                &mut GameState::Playing(ref mut data) => {
                    let inp = read_line_space();
                    if inp.len() != 1 {
                        println!("Please type only 1 letter.");
                    } else {
                        let ch = inp.chars().next().unwrap();
                        data.guess(ch);
                        if data.lives == 0 {
                            change = Some(GameState::Lost(data.word.iter().cloned().collect()));
                            self.save.update(GameState::Lost(String::from("")), &self.diff);
                        } else if data.left == 0 {
                            change = Some(GameState::Won(data.word.iter().cloned().collect()));
                            self.save.update(GameState::Won(String::from("")), &self.diff);
                        }
                    }
                }
                &mut GameState::Menu => {
                    let inp = read_line_space();
                    if inp == String::from("n") {
                        return;
                    } else if inp == String::from("y") {
                        change = Some(GameState::Playing(GameData::new(self.words.rand_word(), self.diff.get_lives())));
                    } else if inp == String::from("s") {
                        change = Some(GameState::Settings);
                    } else {
                        println!("Unknown command.");
                    }
                }
                &mut GameState::Settings => {
                    let inp = read_line_space();
                    change = Some(GameState::Menu);
                    if inp == String::from("e") {
                        self.diff = Difficulty::Easy;
                    } else if inp == String::from("m") {
                        self.diff = Difficulty::Medium;
                    } else if inp == String::from("h") {
                        self.diff = Difficulty::Hard;
                    } else if inp == String::from("s") {
                        self.save.print();
                    } else if inp == String::from("r") {
                        println!("Are you sure you want to reset your progress? [y/n]");
                        let inp2 = read_line();
                        if inp2 == String::from("y") {
                            self.save.reset();
                            println!("Progress has been reset.");
                        } else {
                            println!("Progress was not reset.");
                        }
                    } else {
                        println!("Unknown command.");
                        change = None;
                    }
                }
                _ => {
                    change = Some(GameState::Menu);
                }
            }
            if let Some(ch) = change {
                self.state = ch;
            }
        }
    }
}
