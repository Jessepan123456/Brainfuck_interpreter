use std::{collections::HashMap, io::stdin};

fn main() {
    let mut brainfuck: Vec<i8> = vec![0, 0, 0, 0, 0];
    let mut ptr = 0;

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read");

    let mut my_stack: Vec<usize> = Vec::new();
    let mut index = 0;
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut i = 0;

    //Turn program into a vec
    let program: Vec<char> = input.chars().collect();

    //Inital Scan
    for command in input.chars() {
        match command {
            '[' => my_stack.push(index),
            ']' => {
                let start = my_stack.pop().unwrap();
                map.insert(start, index);
                map.insert(index, start);
            }
            _ => {}
        }
        index += 1
    }

    //Scan
    while i < program.len() {
        let mut jump = false;
        let command = program[i];

        match command {
            '[' => {
                if brainfuck[ptr] == 0 {
                    //Destination
                    i = *map.get(&i).unwrap();
                    jump = true
                }
            }
            ']' =>  {
                if brainfuck[ptr] != 0{
                    //Start point
                    i = *map.get(&i).unwrap();
                    jump = true
                }
            }
            '+' => {
                brainfuck[ptr] += 1;
            }
            '-' => {
                brainfuck[ptr] -= 1;
            }
            '>' => ptr += 1,
            '<' => ptr -= 1,
            '.' => {
                let ascii = brainfuck[ptr] as u8 as char;
                println!("{}", ascii);
            }
            ',' => {
                let mut ascii_char = String::new();
                stdin().read_line(&mut ascii_char).expect("Failed to read");
                let ascii_value = ascii_char.as_bytes()[0];
                brainfuck[ptr] = ascii_value as i8;
            }
            _ => {}
        }
        if !jump {
            i += 1
        }
    }

    println!("{:?}", map);
    println!("{:?}", brainfuck);
}
