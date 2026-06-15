use std::io::stdin;

use crate::game::{game1, game2, game3, game4};
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
    2. Pattern Game\n
    3. Survival Game\n
    4. Puzzle Game\n
    5. brainfuck interpreter
    "
    );
    let choice = input_option();

    match choice.as_str() {
        "1" => game1(),
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
