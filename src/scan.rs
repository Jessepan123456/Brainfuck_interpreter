use std::collections::HashMap;
use std::io::stdin;

pub fn run(
    input: &str,
    map: &mut HashMap<usize, usize>,
    my_stack: &mut Vec<usize>,
    debug_option: &mut String,
    program: &Vec<char>,
    i: &mut usize,
    ptr: &mut usize,
    brainfuck: &mut Vec<u8>,
) {
    //Inital Scan
    if inital_scan(&input, map, my_stack) {
        //Debug Option
        println!("Do you want Debug Mode(Y for yes)");
        stdin()
            .read_line( debug_option)
            .expect("Failed to read");
    } else {
        return;
    }

    //Actual Scan
    while *i < program.len() {
        let command = program[*i];
        if debug_option.trim() == "Y" {
            println!(
                "IP: {} CMD: {} PTR: {} MEM: {:?}",
                i, command, ptr, brainfuck,
            );
        }

        scan_result(brainfuck, ptr, &command, i, &map);
    }
    println!("{:?}", brainfuck);
}


pub fn inital_scan(
    input: &str,
    map: &mut HashMap<usize, usize>,
    my_stack: &mut Vec<usize>,
) -> bool {
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
                    println!("Missing [, so it incomplete");
                    return false;
                }
            }
            _ => {}
        }
        index += 1
    }

    if count != 0 {
        println!("[ is incomplete because missing ]");
        return false;
    }

    return true;
}

pub fn scan_result(
    brainfuck: &mut Vec<u8>,
    ptr: &mut usize,
    command: &char,
    i: &mut usize,
    map: &HashMap<usize, usize>,
) {
    match command {
        '[' => {
            if brainfuck[*ptr] == 0 {
                //Destination
                if let Some(m) = map.get(&i) {
                    *i = *m
                } else {
                    return;
                }
            }
        }
        ']' => {
            if brainfuck[*ptr] != 0 {
                //Start point
                if let Some(m) = map.get(&i) {
                    *i = *m
                } else {
                    return;
                }
            }
        }
        '+' => {
            brainfuck[*ptr] += 1;
        }
        '-' => {
            brainfuck[*ptr] -= 1;
        }
        '>' => {
            *ptr += 1;
            if *ptr >= brainfuck.len() {
                brainfuck.push(0)
            }
        }
        '<' => {
            if *ptr > 0 {
                *ptr -= 1
            }
        }
        '.' => {
            let ascii = brainfuck[*ptr] as u8 as char;
            println!("{}", ascii);
        }
        ',' => {
            let mut ascii_char = String::new();
            stdin().read_line(&mut ascii_char).expect("Failed to read");
            let ascii_value = ascii_char.as_bytes()[0];
            brainfuck[*ptr] = ascii_value as u8;
        }
        _ => {}
    }
    *i += 1;
}
