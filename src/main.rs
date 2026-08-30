use std::collections::HashMap;
use std::env;
use std::hash::Hash;
use std::io;
use std::process;

fn check_all_true(vec: &Vec<bool>) -> bool {
    if vec.len() == 0 {
        return false;
    }
    vec.iter().all(|e| *e == true)
}

// currently handles \d \input -> 1 apple
fn pattern_parser(input_line: &str, pattern: &str) -> bool {
    let input_parts: Vec<&str> = input_line.splitn(2, " ").collect();
    let pattern_parts: Vec<&str> = pattern.splitn(2, " ").collect();

    if input_parts.len() == pattern_parts.len() && input_parts.len() == 2 {
        let (input_1, input_2) = (input_parts[0], input_parts[1]);
        let (pattern_1, pattern_2) = (pattern_parts[0], pattern_parts[1]);

        let input_2_split = input_2.split(" ").collect::<Vec<&str>>();
        let mut res: Vec<bool> = Vec::new();
        if input_2_split.len() > 1 {
            println!("now spilt inputs are..{:?}", input_2_split);
            let re_spilt = input_line.split(" ").collect::<Vec<&str>>();
            for (key, val) in re_spilt.iter().enumerate() {
                println!("key {} val {} matching {}", key, val, pattern_1);
                check_input_pattern(val, pattern_1, &mut res, "\\d");
                println!("now res is {:?}", res);
                if res.len() == 0 {
                    return false;
                }
                if check_all_true(&res) {
                    // reach the end then false
                    println!("check passed");
                    if key == re_spilt.len() - 1 {
                        return false;
                    }
                    // get next index and if it match then pass
                    let next = re_spilt[key + 1];
                    res = Vec::new();
                    check_input_pattern(next, pattern_2, &mut res, "\\w");
                    return check_all_true(&res);
                }

                res = Vec::new(); // re init bool vec
            }

            return false;
        }

        // UPDATE RES VALUE
        res = Vec::new();
        check_input_pattern(input_1, pattern_1, &mut res, "\\d");
        check_input_pattern(input_2, pattern_2, &mut res, "\\w");

        match res.len() {
            0 => {
                return false;
            }
            _ => {
                return check_all_true(&res);
            }
        }
    }

    return false;
}

fn check_first_input_pattern(
    input_1: &str,
    pattern_1: &str,
    map: &HashMap<&str, &str>,
    res: &mut Vec<bool>,
) -> Option<bool> {
    let pt_count_1: Vec<_> = pattern_1.match_indices("\\d").map(|(i, _)| i).collect();
    if pt_count_1.len() == 1 {
        for (key, val) in map.into_iter() {
            let matched = match_pattern(key, val);
            println!("{} / {} / matching,{}", key, val, matched);
            res.push(matched);
        }
    } else {
        println!(
            "more than one pattern {} {:?} {}",
            pattern_1, pt_count_1, input_1
        );
        let pt_count_1_len = pt_count_1.len();
        if pt_count_1_len != input_1.len() {
            return Some(false); // pattern length does not match input length
        } else {
            println!("matching...{}", input_1);
            for i in [0..input_1.len()] {
                let matched = match_pattern(&input_1[i], "\\d");
                res.push(matched);
            }
        }
    }
    None
}

//this function works with \d\apple \d\d\d \w\w\ws etc
fn check_input_pattern(
    input: &str,
    pattern: &str,
    res: &mut Vec<bool>,
    input_ptn: &str,
) -> Option<bool> {
    // base case input == pattern
    if input.contains(pattern) {
        res.push(true);
        return Some(true);
    }
    // handle plural
    if pattern.ends_with("s") && !input.ends_with("s") {
        res.push(false);
        return Some(false);
    }

    let mut pt_count: Vec<_> = pattern.match_indices(input_ptn).map(|(i, _)| i).collect();
    println!(
        "pt_count {:?} and input {} and pattern {}",
        pt_count, input, pattern
    );
    if pt_count.len() == 1 {
        let matched = match_pattern(input, pattern);
        println!("{input} matched one pattern");
        res.push(matched);
        return Some(true);
    } else {
        //println!("more than one pattern {} {:?} {}", pattern, pt_count, input);
        let pt_count_len = pt_count.len();
        if pt_count_len == input.len() {
            println!("matching...{}", input);
            for i in [0..input.len()] {
                let matched = match_pattern(&input[i], input_ptn);
                res.push(matched);
                return Some(true);
            }
        } else if pt_count_len == 0 {
            println!("current matching: ...{:?}", pt_count);
            res.push(false); //empty string error
            return Some(false);
        } else if pt_count_len != 0 && pt_count_len < input.len() {
            let last_ptn_pos = pt_count.len();
            let last_ptn_start = pt_count.last().unwrap();
            let final_ptn = &pattern[last_ptn_start + input_ptn.len()..pattern.len()];
            let input_not_matched = &input[last_ptn_pos..];
            let matchable_input = &input[0..last_ptn_pos + 1];

            println!(
                "DBG last pt: {last_ptn_pos}, final_ptn:  {final_ptn}, input_not_matched: {input_not_matched}"
            );
            if final_ptn != input_not_matched {
                res.push(false);
                return Some(false);
            } else {
                println!(
                    "last pt: {last_ptn_pos}, final_ptn:  {final_ptn}, input_not_matched: {input_not_matched}"
                );
                for i in [0..matchable_input.len()] {
                    let matched = match_pattern(&matchable_input[i], input_ptn);
                    res.push(matched);
                    return Some(true);
                }
            }
        }
    }
    None
}

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
                        println!("the slice is: {:?}", slice);
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
        _ => {
            // handle cases e.g. apple==apple
            if input_line.contains(pattern) {
                return true;
            } else {
                return false;
            }
        }
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

    //handle single input
    if input_line.len() == 1 {
        if match_pattern(&input_line, &pattern) {
            process::exit(0)
        } else {
            process::exit(1)
        }
    } else {
        if pattern_parser(&input_line, &pattern) {
            println!("matching results achieved!");
            process::exit(0)
        } else {
            process::exit(1)
        }
    }
}
