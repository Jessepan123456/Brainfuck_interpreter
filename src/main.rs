use rand::RngExt;
use std::io::Write;
use std::time::Instant;
use std::{collections::HashMap, io::stdin, thread::sleep, time::Duration};

use crate::game::{game1, game2};
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
    4. Puzzle\n
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

// 1. Maze
// 2. Survival
// 3. Reaction
// 4. Puzzle

fn game3() {
    println!(
        "
    Cell0 GameState(0-Waiting 1-Start 2-Finish)\n
    Cell1 Result(0-alive 1-dead)\n
    Cell2 Health\n
    Cell3 Damage\n
    Cell4 Time you surived
    "
    );

    println!("Type S to start: ");

    let mut survival_array: Vec<u8> = vec![0; 5];
    let mut ptr = 0;
    let mut my_stack: Vec<usize> = Vec::new();
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut i = 0;

    survival_array[0] = 1;
    survival_array[2] = 10;
    survival_array[3] = rand::rng().random_range(1..3);

    if input_option() != "S" {
        println!("Try Again");
        return;
    }

    println!("Starting...");
    let alive_time = Instant::now();

    while survival_array[2] > 0 {
        sleep(Duration::from_secs(1));
        println!("HP : {}", survival_array[2]);
        survival_array[2] -= survival_array[3];
    }

    survival_array[4] = alive_time.elapsed().as_secs() as u8;
    survival_array[1] = 1;

    println!("{:?}", survival_array);
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

    let mut Puzzle_array: Vec<u8> = vec![0; 5];
    let mut ptr = 0;
    let mut my_stack: Vec<usize> = Vec::new();
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut i = 0;

    Puzzle_array[0] = 1;
    Puzzle_array[2] = rand::rng().random_range(1..120);

    if input_option() != "S" {
        println!("Try Again");
        return;
    }
        
    println!("Starting Puzzle...");
    println!("{:?}", Puzzle_array);
    println!("Target : {}, Current : {}, Ptr : {}", Puzzle_array[2], Puzzle_array[3], ptr);
}
