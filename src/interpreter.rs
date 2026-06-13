use crate::InputMode;
use crate::input_option;
use crate::scan;

use std::collections::HashMap;

pub fn interpreter() {
    //Main Array
    let mut interpreter: Vec<u8> = vec![0, 0, 0, 0, 0];
    let mut ptr = 0;
    let mode = InputMode::Ascii;

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
