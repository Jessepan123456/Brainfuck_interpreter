use std::io::stdin;

fn main() {
    let mut brainfuck: Vec<u8> = vec![0, 0, 0, 0, 0];
    let mut ptr = 0;

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read");

    for command in input.chars() {
        match command {
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
                brainfuck[ptr] = ascii_value;
            }
            _ => {}
        }
    }
    println!("{:?}", brainfuck);
}
