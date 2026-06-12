use rand::{Rng, RngExt};
use std::{collections::HashMap, io::stdin, thread::sleep, time::Duration};

use crate::scan::scan_result;
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

fn game1() {
    println!(
        "
    Cell0 GameState(0-Waiting 1-Start 2-Finish)\n
    Cell1 Result(-1-wrong 1-correct)\n
    Cell2 Target Value\n
    Cell3 Reaction Score\n
    Type , to guess when Cell0 turn 1
    "
    );
    println!("Type S to start: ");

    if input_option() == "S" {
        //Waiting
        let mut reaction_array: Vec<u8> = vec![0, 0, 0, 0];
        let mut ptr = 0;
        let _my_stack: Vec<usize> = Vec::new();
        let mut map: HashMap<usize, usize> = HashMap::new();
        let mut i = 0;
        let ascii_mode = InputMode::Ascii;
        let num_mode = InputMode::Number;
        println!("{:?}", reaction_array);

        //Start Signal
        let mut rng = rand::rng();
        let start_timer = rng.random_range(1..10);
        sleep(Duration::from_secs(start_timer));

        //Target Value
        let mut target = rand::rng();
        reaction_array[2] = target.random_range(1..=120);

        scan_result(
            &mut reaction_array,
            &mut ptr,
            &'+',
            &mut i,
            &mut map,
            &ascii_mode,
        );

        scan_result(
            &mut reaction_array,
            &mut ptr,
            &',',
            &mut i,
            &mut map,
            &ascii_mode,
        );

        println!("{:?}", reaction_array)
    } else {
        println!("Try again");
        return;
    }
}

fn game2() {}

fn game3() {}

fn game4() {}

fn interpreter() {
    //Main Array
    let mut interpreter: Vec<u8> = vec![0, 0, 0, 0, 0];
    let mut ptr = 0;
    let mode = InputMode::Number;

    println!(
        "
    Instruction:\n
    + for increment\n
    - for decrement\n
    > for right\n
    < for left\n
    . for int to char\n
    , for char to int\n
    [ left bracket\n
    ] right bracket\n
    "
    );
    //Input
    let input = &input_option();
    let mut debug_option = String::new();

    //File Reader
    // let content = match fs::read_to_string("brain.txt") {
    //     Ok(c) => c,
    //     Err(_) => {
    //         println!("Failed to read from File");
    //         return;
    //     }
    // };

    //For Mapping
    let mut my_stack: Vec<usize> = Vec::new();
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut i = 0;

    //Turn program into a vec
    let program: Vec<char> = input.chars().filter(|c| "+-><,.[]".contains(*c)).collect();

    scan::run(
        &input,
        &mut map,
        &mut my_stack,
        &mut debug_option,
        &program,
        &mut i,
        &mut ptr,
        &mut interpreter,
        mode,
    );
}
