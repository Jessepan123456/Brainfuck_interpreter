use rand::RngExt;
use std::time::Instant;
use std::{collections::HashMap, io::stdin, thread::sleep, time::Duration};

use crate::game::run_game1;
use crate::interpreter::interpreter;
use crate::scan::{inital_scan, scan_result};

mod game;
mod interpreter;
mod scan;
mod test;

enum InputMode {
    Ascii,
    Number,
}

fn main() {
    // -- Main --
    println!(
        "
    Games\n
    1. Reaction Game\n
    2. Counter Game\n
    3. Movement Game\n
    4. Puzzle\n
    5. brainfuck interpreter
    "
    );
    let choice = input_option();

    match choice.as_str() {
        "1" => run_game1(),
        "2" => game2(),
        "3" => game3(),
        "4" => game4(),
        "5" => interpreter(),
        _ => return,
    }
}

//Helper Method
//Terminal Option
fn input_option() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read");
    return input.trim().to_string();
}

//+++[-]
//++[>++<-]
//++[>++[>+<-]<-]

// 1. Maze
// 2. Survival
// 3. Reaction
// 4. Puzzle

fn game2() {
    println!(
        "
    Cell0 GameState(0-Waiting 1-Start 2-Finish)\n
    Cell1 Result(0-wrong 1-correct)\n
    Cell2 Pattern length\n
    Cell3 Player final pointer position\n
    Cell4 Score
    "
    );

    println!("Type S to start: ");
}

fn game3() {
    println!(
        "
    Cell0 GameState(0-Waiting 1-Start 2-Finish)\n
    Cell1 Result(0-alive 1-dead)\n
    Cell2 Health\n
    Cell3 Damage\n
    Cell4 Round survived
    "
    );

    println!("Type S to start: ");
}

fn game4() {
    println!(
        "
    Cell0 GameState(0-Waiting 1-Start 2-Finish)\n
    Cell1 Result(0-wrong 1-wrong)\n
    Cell2 Target Value\n
    Cell3 Working Value\n
    Cell4 Steps used
    "
    );

    println!("Type S to start: ");
}
