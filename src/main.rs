extern crate hangman;

use hangman::Game;
use hangman::SaveGame;

fn main() {
    let save = SaveGame::new(String::from("savegame"));
    let mut game = Game::new(String::from("knuth"), save);
    game.run();
}
