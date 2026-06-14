use rand::RngExt;
use std::time::Instant;
use std::{collections::HashMap, thread::sleep, time::Duration};

use crate::InputMode;
use crate::input_option;
use crate::{inital_scan, scan_result};

pub fn game1() {
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

    //Target Value
    reaction_array[2] = rand::rng().random_range(1..=120);
    //Start
    reaction_array[0] = 1;

    if input_option() != "S" {
        println!("Try Again");
        return;
    }

    println!("Starting...");

    run_game1(
        &mut reaction_array,
        &mut map,
        &mut my_stack,
        &mut i,
        &mut ptr,
    );
}

fn run_game1(
    reaction_array: &mut Vec<u8>,
    map: &mut HashMap<usize, usize>,
    my_stack: &mut Vec<usize>,
    i: &mut usize,
    ptr: &mut usize,
) {
    //Start Signal
    let start_timer = rand::rng().random_range(1..10);
    sleep(Duration::from_secs(start_timer));

    let start_time = Instant::now();
    println!("{:?}", reaction_array);

    //Input Guess
    let commands = ">>>,";

    inital_scan(commands, map, my_stack);

    let program: Vec<char> = commands.chars().collect();
    while *i < program.len() {
        let command = program[*i];
        scan_result(reaction_array, ptr, &command, i, map, &InputMode::Number);
    }

    let reaction_time = start_time.elapsed().as_millis();
    reaction_array[4] = reaction_time as u8;
    reaction_array[0] += 1;

    if reaction_array[2] == reaction_array[3] {
        reaction_array[1] = 1;
    }

    println!("{:?}", reaction_array);
}

pub fn game2() {
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
    let mut pattern_array: Vec<u8> = vec![0; 5];
    let mut ptr = 0;
    let mut _map: HashMap<usize, usize> = HashMap::new();
    let mut i = 0;

    //Generate Pattern
    let len = rand::rng().random_range(3..10);
    let mut pattern = String::new();

    for _p in 0..len {
        let c = if rand::rng().random_bool(0.6) {
            '>'
        } else {
            '<'
        };
        pattern.push(c);
    }

    pattern_array[0] = 1;
    pattern_array[2] = len;

    for command in pattern.chars() {
        scan_result(
            &mut pattern_array,
            &mut ptr,
            &command,
            &mut i,
            &_map,
            &InputMode::Number,
        );
    }

    let mut expected_ptr = ptr;
    ptr = 0;

    if input_option() != "S" {
        println!("Try Again");
        return;
    }

    println!("Showing...");
    run_game2(
        pattern,
        &mut pattern_array,
        &mut ptr,
        &mut i,
        &mut _map,
        &mut expected_ptr,
    );
}

fn run_game2(
    pattern: String,
    pattern_array: &mut Vec<u8>,
    ptr: &mut usize,
    i: &mut usize,
    map: &mut HashMap<usize, usize>,
    expected_ptr: &mut usize,
) {
    println!("{}", pattern);

    //Showing timer before remove
    sleep(Duration::from_secs(3));
    println!("\x1B[2J\x1B[1;1H");

    println!("{:?}", pattern_array);

    let user_input = input_option();

    for command in user_input.chars() {
        scan_result(pattern_array, ptr, &command, i, &map, &InputMode::Number);
    }

    pattern_array[3] = *ptr as u8;

    if ptr == expected_ptr {
        pattern_array[1] = 1;
        pattern_array[4] += 100;
    }

    pattern_array[0] = 2;

    println!("{:?}", pattern_array);
}
