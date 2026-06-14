use crate::InputMode;
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
    interpreter: &mut Vec<u8>,
    mode: InputMode,
) {
    //Inital Scan
    if inital_scan(&input, map, my_stack) {
        //Debug Option
        println!("Do you want Debug Mode(Y for yes)");
        stdin().read_line(debug_option).expect("Failed to read");
    } else {
        return;
    }

    //Actual Scan
    while *i < program.len() {
        let command = program[*i];
        if debug_option.trim() == "Y" {
            println!(
                "IP: {} CMD: {} PTR: {} MEM: {:?}",
                i, command, ptr, interpreter,
            );
        }

        scan_result(interpreter, ptr, &command, i, &map, &mode);
    }
    println!("{:?}", interpreter);
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
    interpreter: &mut Vec<u8>,
    ptr: &mut usize,
    command: &char,
    i: &mut usize,
    map: &HashMap<usize, usize>,
    mode: &InputMode,
) {
    match command {
        '[' => {
            if interpreter[*ptr] == 0 {
                //Destination
                if let Some(m) = map.get(&i) {
                    *i = *m
                } else {
                    return;
                }
            }
        }
        ']' => {
            if interpreter[*ptr] != 0 {
                //Start point
                if let Some(m) = map.get(&i) {
                    *i = *m
                } else {
                    return;
                }
            }
        }
        '+' => {
            interpreter[*ptr] += 1;
        }
        '-' => {
            interpreter[*ptr] -= 1;
        }
        '>' => {
            *ptr += 1;
            if *ptr >= interpreter.len() {
                interpreter.push(0)
            }
        }
        '<' => {
            if *ptr > 0 {
                *ptr -= 1
            }
        }
        '.' => {
            let ascii = interpreter[*ptr] as u8 as char;
            println!("{}", ascii);
        }
        ',' => {
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed to read");

            match mode {
                InputMode::Ascii => {
                    let ascii_value = input.as_bytes()[0];
                    interpreter[*ptr] = ascii_value as u8;
                }
                InputMode::Number => {
                    let input = input.trim();
                    if let Ok(v) = input.parse::<u8>() {
                        interpreter[*ptr] = v
                    } else {
                        println!("Failed to parse");
                    }
                }
            }
        }
        _ => {}
    }
    *i += 1;
}
