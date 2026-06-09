use std::{collections::HashMap, io::stdin};

fn main() {
    //Main Array
    let mut brainfuck: Vec<u8> = vec![0, 0, 0, 0, 0];
    let mut ptr = 0;

    //Input
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read");
    let input = input.trim();

    //For "[", "]"
    let mut my_stack: Vec<usize> = Vec::new();
    let mut index = 0;
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut i = 0;
    let mut count = 0;

    // //Turn program into a vec
    let program: Vec<char> = input.chars().filter(|c| "+-><,.[]".contains(*c)).collect();

    //Inital Scan
    for command in input.chars() {
        match command {
            '[' => {
                my_stack.push(index);
                count += 1;
            }
            ']' => {
                if let Some(start) = my_stack.pop() {
                    map.insert(start, index);
                    map.insert(index, start);
                    count -= 1;
                } else {
                    println!("Doesn't have [, so it incomplete");
                    return;
                }
            }
            _ => {}
        }
        index += 1
    }

    if map.is_empty() || count != 0 {
        println!("[ is incomplete");
        return;
    }
    
    println!("{:?}", map);

    //Scan
    while i < program.len() {
        let command = program[i];

        match command {
            '[' => {
                if brainfuck[ptr] == 0 {
                    //Destination
                    i = *map.get(&i).unwrap();
                }
            }
            ']' => {
                if brainfuck[ptr] != 0 {
                    //Start point
                    i = *map.get(&i).unwrap();
                }
            }
            '+' => {
                brainfuck[ptr] += 1;
            }
            '-' => {
                brainfuck[ptr] -= 1;
            }
            '>' => ptr += 1,
            '<' => {
                if ptr > 0 {
                    ptr -= 1
                }
            }
            '.' => {
                let ascii = brainfuck[ptr] as u8 as char;
                println!("{}", ascii);
            }
            ',' => {
                let mut ascii_char = String::new();
                stdin().read_line(&mut ascii_char).expect("Failed to read");
                let ascii_value = ascii_char.as_bytes()[0];
                brainfuck[ptr] = ascii_value as u8;
            }
            _ => {}
        }
        i += 1;
    }
    println!("{:?}", brainfuck);
}
