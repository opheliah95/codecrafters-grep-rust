use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    match pattern {
        "\\d" => input_line.chars().any(|e| e.is_ascii_digit()),
        "d" => input_line.starts_with(|s: char| s.is_ascii_alphabetic()),
        "\\w" => input_line
            .chars()
            .any(|e| e.is_ascii_alphanumeric() || e == '_'),
        ptn if pattern.starts_with("[") && pattern.ends_with("]") => {
            if let Some(content) = ptn.get(1..ptn.len() - 1) {
                if content.len() == 0 {
                    return false;
                }
                // create content vec
                let content_vec: Vec<char> = content.chars().collect();

                //negative char check
                if content.chars().nth(0) == Some('^') {
                    if content.len() > 1 {
                        let slice = &content_vec[1..];
                        //println!("the slice is: {:?}", slice);
                        return input_line.chars().any(|e| !slice.contains(&e));
                    }
                    return false;
                } else {
                    //positive char check
                    return input_line.chars().any(|e| content_vec.contains(&e));
                }
            } else {
                return false;
            }
        }
        _ => false,
    }
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    //TODO: Uncomment the code below to pass the first stage
    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
