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
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut i = 0;


    // //Turn program into a vec
    let program: Vec<char> = input.chars().filter(|c| "+-><,.[]".contains(*c)).collect();

    //Inital Scan
    inital_scan(input, &mut map, &mut my_stack);
    
    println!("{:?}", map);

    //Debug Option
    println!("Do you want Debug Mode(Y for yes)");
    let mut debug = String::new();
    stdin().read_line(&mut debug).expect("Failed to read");
    let debug = debug.trim();

    //Scan
    while i < program.len() {
        let command = program[i];
        if debug == "Y" {
            println!("IP: {} CMD: {} PTR: {} MEM: {:?}",
                i,
                command,
                ptr,
                brainfuck,
            );
        }

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

fn inital_scan( input : &str, map : &mut HashMap<usize, usize>, my_stack : &mut Vec<usize>){
    let mut count = 0;
    let mut index = 0;
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

    if count != 0 {
        println!("[ is incomplete because of ]");
        return;
    }
}

fn scan_option(){
}

fn scan() {

}