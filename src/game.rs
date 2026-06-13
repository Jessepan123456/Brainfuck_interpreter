use rand::RngExt;
use std::time::Instant;
use std::{collections::HashMap, thread::sleep, time::Duration};

use crate::InputMode;
use crate::input_option;
use crate::{inital_scan, scan_result};

pub fn run_game1() {
    println!(
        "
    Cell0 GameState(0-Waiting 1-Start 2-Finish)\n
    Cell1 Result(0-wrong 1-correct)\n
    Cell2 Target Value\n
    Cell3 Your Value\n
    Cell4 Reaction Score\n
    Type , to guess when Cell0 turn 1
    "
    );
    
    println!("Type S to start: ");
    let mut reaction_array: Vec<u8> = vec![0; 5];
    let mut ptr = 0;
    let mut my_stack: Vec<usize> = Vec::new();
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut i = 0;
    let num_mode = InputMode::Number;

    //Target Value
    reaction_array[2] = rand::rng().random_range(1..=120);
    //Start
    reaction_array[0] = 1;

    if input_option() != "S" {
        println!("Try Again");
        return;
    }

    println!("Starting...");

    //Start Signal
    let start_timer = rand::rng().random_range(1..10);
    sleep(Duration::from_secs(start_timer));

    let start_time = Instant::now();
    println!("{:?}", reaction_array);

    //Input Guess
    let commands = ">>>,";

    inital_scan(commands, &mut map, &mut my_stack);

    let program: Vec<char> = commands.chars().collect();
    while i < program.len() {
        let command = program[i];
        scan_result(
            &mut reaction_array,
            &mut ptr,
            &command,
            &mut i,
            &mut map,
            &num_mode,
        );
    }

    let reaction_time = start_time.elapsed().as_millis();
    reaction_array[4] = reaction_time as u8;

    if reaction_array[2] == reaction_array[3] {
        reaction_array[1] = 1;
    }

    println!("{:?}", reaction_array);
}

