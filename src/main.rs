use std::{collections::HashMap, io::stdin};
mod scan;

fn main() {
    //Main Array
    let mut brainfuck: Vec<u8> = vec![0, 0, 0, 0, 0];
    let mut ptr = 0;

    //Input
    let input = &input_option();
    let mut debug_option = String::new();

    //For Mapping
    let mut my_stack: Vec<usize> = Vec::new();
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut i = 0;

    //File Reader
    // let content = match fs::read_to_string("brain.txt") {
    //     Ok(c) => c,
    //     Err(_) => {
    //         println!("Failed to read from File");
    //         return;
    //     }
    // };

    //Turn program into a vec
    let program: Vec<char> = input.chars().filter(|c| "+-><,.[]".contains(*c)).collect();

    scan::run(&input, &mut map, &mut my_stack, &mut debug_option, &program, &mut i, &mut ptr, &mut brainfuck);
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
