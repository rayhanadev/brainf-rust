use std::env;
use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut contents = String::new();

    match args.get(1) {
        Some(file_name) => {
            let mut file = File::open(file_name).expect("File not found");
            file.read_to_string(&mut contents)
                .expect("Error reading file");
        }
        None => {
            println!("No file name provided");
        }
    }

    let mut stack: Vec<i32> = Vec::new();
    stack.resize(1, 0);

    let mut pointer = 0;

    let mut i = 0;

    while i < contents.len() {
        let ch = contents.chars().nth(i).unwrap();
        match ch {
            '<' => {
                if pointer > 0 {
                    pointer -= 1;
                }
            }
            '>' => {
                pointer += 1;
                if stack.len() <= pointer {
                    stack.push(0);
                }
            }
            '+' => {
                stack[pointer] += 1;
            }
            '-' => {
                if stack[pointer] > 0 {
                    stack[pointer] -= 1;
                }
            }
            '.' => {
                print!("{}", stack[pointer] as u8 as char);
            }
            ',' => {
                let mut x = String::new();
                io::stdin().read_line(&mut x).expect("Failed to read line");
                stack[pointer] = match x.trim().parse() {
                    Ok(num) => num,
                    Err(_) => 0,
                };
                // stack[pointer] = c as i32;
                // c += 1;
            }
            '[' => {
                if stack[pointer] == 0 {
                    let mut count = 1;
                    while count > 0 {
                        i += 1;
                        let ch = contents.chars().nth(i).unwrap();
                        if ch == '[' {
                            count += 1;
                        } else if ch == ']' {
                            count -= 1;
                        }
                    }
                }
            }
            ']' => {
                if stack[pointer] != 0 {
                    let mut count = 1;
                    while count > 0 {
                        i -= 1;
                        let ch = contents.chars().nth(i).unwrap();
                        if ch == '[' {
                            count -= 1;
                        } else if ch == ']' {
                            count += 1;
                        }
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
}
