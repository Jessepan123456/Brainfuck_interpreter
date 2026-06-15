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
    println!("Pattern : {}", pattern);

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

pub fn game3() {
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
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut i: usize = 0;
    let commands = "++";
    let hp_cooldown = 0;
    let block_cooldown = 0;

    survival_array[0] = 1;
    survival_array[2] = 10;
    survival_array[3] = rand::rng().random_range(2..4);

    if input_option() != "S" {
        println!("Try Again");
        return;
    }

    println!("Starting...");

    run_game3(
        &mut survival_array,
        &mut i,
        &mut map,
        hp_cooldown,
        block_cooldown,
        commands,
    );
}

fn run_game3(
    survival_array: &mut Vec<u8>,
    i: &mut usize,
    map: &mut HashMap<usize, usize>,
    mut hp_cooldown: i32,
    mut block_cooldown: i32,
    commands: &str,
) {
    let alive_time = Instant::now();

    while survival_array[2] > 0 {
        sleep(Duration::from_secs(1));
        println!("HP : {}, Damage : {}", survival_array[2], survival_array[3]);

        //User Input
        println!(
            "Choose:\n
        1. Heal(+2HP)\n
        2. Block(no damage)\n
        3. Do nothing
        "
        );

        let _user_input: u8;
        if let Ok(u) = input_option().parse::<u8>() {
            _user_input = u
        } else {
            println!("Failed to parse");
            return;
        }

        if hp_cooldown > 0 {
            hp_cooldown -= 1
        }
        if block_cooldown > 0 {
            block_cooldown -= 1
        }

        match _user_input {
            1 => {
                if hp_cooldown == 0 {
                    for command in commands.chars() {
                        scan_result(
                            survival_array,
                            &mut 2,
                            &command,
                            i,
                            &map,
                            &InputMode::Number,
                        );
                    }
                    hp_cooldown = 3;
                    survival_array[2] = survival_array[2].saturating_sub(survival_array[3]);
                } else {
                    println!("HP Potion on cool down for {} turn", hp_cooldown);
                    survival_array[2] = survival_array[2].saturating_sub(survival_array[3]);
                }
            }
            2 => {
                if block_cooldown == 0 {
                    block_cooldown = 2;
                } else {
                    println!("Block on cool down for {} turn", block_cooldown);
                    survival_array[2] = survival_array[2].saturating_sub(survival_array[3]);
                }
            }
            3 => {
                survival_array[2] = survival_array[2].saturating_sub(survival_array[3]);
            }
            _ => {
                return;
            }
        }
    }

    survival_array[4] = alive_time.elapsed().as_secs() as u8;
    survival_array[1] = 1;

    println!("{:?}", survival_array);
}

pub fn game4() {
    println!(
        "
    Cell0 GameState(0-Waiting 1-Start 2-Finish)\n
    Cell1 Result(0-wrong 1-right)\n
    Cell2 Target Value\n
    Cell3 Working Value\n
    Cell4 Steps used\n
    You can use Ascii Value but if you want it to be harder try using without it
    "
    );

    println!("Type S to start: ");

    let mut puzzle_array: Vec<u8> = vec![0; 5];
    let mut my_stack: Vec<usize> = Vec::new();
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut i: usize = 0;
    let mode = &InputMode::Ascii;

    puzzle_array[0] = 1;
    puzzle_array[2] = rand::rng().random_range(1..=120);

    if input_option() != "S" {
        println!("Try Again");
        return;
    }

    println!("Starting Puzzle...");
    run_game4(&mut puzzle_array, &mut i, &mut map, &mut my_stack, mode);
}

fn run_game4(
    puzzle_array: &mut Vec<u8>,
    i: &mut usize,
    map: &mut HashMap<usize, usize>,
    my_stack: &mut Vec<usize>,
    mode: &InputMode,
) {
    sleep(Duration::from_secs(3));

    println!("{:?}", puzzle_array);
    println!(
        "Target : {}, Current : {}",
        puzzle_array[2], puzzle_array[3]
    );

    let user_input = input_option();

    let program: Vec<char> = user_input
        .chars()
        .filter(|c| "+-><,.[]".contains(*c))
        .collect();

    if !inital_scan(&user_input, map, my_stack) {
        return;
    }

    while *i < program.len() {
        let command = program[*i];

        scan_result(puzzle_array, &mut 3, &command, i, &map, mode);
    }

    if puzzle_array[2] == puzzle_array[3] {
        puzzle_array[1] = 1;
    }

    puzzle_array[0] = 2;
    puzzle_array[4] = program.len() as u8;
    println!("{:?}", puzzle_array);
}
