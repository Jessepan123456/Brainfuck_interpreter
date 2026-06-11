use std::{collections::HashMap, fs, io::stdin};
mod scan;

fn main() {
    //Main Array
    let mut brainfuck: Vec<u8> = vec![0, 0, 0, 0, 0];
    let mut ptr = 0;

    //Input
    // let input = &input_option();
    let mut debug_option = String::new();

    //For Mapping
    let mut my_stack: Vec<usize> = Vec::new();
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut i = 0;

    //File Reader
    let content = match fs::read_to_string("brain.txt") {
        Ok(c) => c,
        Err(_) => {
            println!("Failed to read from File");
            return;
        }
    };

    //Turn program into a vec
    let program: Vec<char> = content
        .chars()
        .filter(|c| "+-><,.[]".contains(*c))
        .collect();

    //Inital Scan
    if scan::inital_scan(&content, &mut map, &mut my_stack) {
        //Debug Option
        println!("Do you want Debug Mode(Y for yes)");
        stdin()
            .read_line(&mut debug_option)
            .expect("Failed to read");
    } else {
        return;
    }

    //Actual Scan
    while i < program.len() {
        let command = program[i];
        if debug_option.trim() == "Y" {
            println!(
                "IP: {} CMD: {} PTR: {} MEM: {:?}",
                i, command, ptr, brainfuck,
            );
        }

        scan::scan_result(&mut brainfuck, &mut ptr, &command, &mut i, &map);
    }
    println!("{:?}", brainfuck);
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
