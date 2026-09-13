use std::env;
use std::f32::consts::LN_10;
use std::io::{self, Read};
use std::process;
mod lib;
use lib::{check_digits, exit_process_errored, remove_start_end, start_end_is_pattern};
mod match_func;
use match_func::{match_pattern, print_single_matching_line};

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");
    let mut input_line = String::new();
    io::stdin().read_to_string(&mut input_line).unwrap();
    let mut pattern = env::args().nth(2).unwrap();
    let split_input_by_space = input_line.split_whitespace().collect::<Vec<&str>>();
    //println!("{input_line} line is: {:?}", split_input_by_space);
    if env::args().nth(1).unwrap() == "-o" {
        pattern = env::args().nth(3).unwrap();

        let mut split_by_full_stop: Vec<&str> = input_line.split(".").collect();
        let split_by_full_stop_cleaned: Vec<String> = split_by_full_stop
            .iter()
            .filter(|a| a.len() > 0)
            .map(|a| a.to_string())
            .collect();
        //println!("res of spilt full stop: {:?}", split_by_full_stop_cleaned);

        let mut res: Vec<String> = Vec::new();

        if split_by_full_stop_cleaned.len() > 1 {
            res = handle_sentence_ptn(&split_by_full_stop_cleaned, pattern.clone());
            if res.len() >= 1 {
                for r in res {
                    let s = r.trim_end();
                    println!("{s}");
                }

                process::exit(0);
            } else {
                //println!("res empty");
                process::exit(1);
            }
        } else {
            let split_ptn_by_space = pattern.split_whitespace().collect::<Vec<&str>>();
            //echo -ne "mango\n!@#$\nbanana\n+++\ntest123" | ./your_program.sh -E '\w+'
            //println!("{:?} vs PTN {:?}", split_input_by_space, split_ptn_by_space);

            if split_ptn_by_space.len() == split_input_by_space.len() {
                res = handle_single_matching_line(&split_input_by_space, split_ptn_by_space);
            } else if split_ptn_by_space.len() != split_input_by_space.len() {
                res = handle_single_ptn_to_spaced_txt(&split_input_by_space, &split_ptn_by_space);
            } else if split_ptn_by_space.len() == 0 || split_ptn_by_space.len() == 0 {
                exit_process_errored();
            }
        }

        let res_len = res.len();
        if res_len >= 1 {
            let res_collected: Vec<String> = res
                .iter()
                .enumerate()
                .map(|(idx, a)| {
                    if idx == res_len - 1 {
                        a.trim_end().to_string()
                    } else {
                        if a.chars().last().unwrap() == 0xA as char {
                            a.to_string()
                        } else {
                            format!("{} ", a)
                        }
                    }
                })
                .collect();
            //println!("{:?}", res_collected);
            println!("{}", res_collected.join(""));
            process::exit(0);
        } else {
            //println!("res empty");
            process::exit(1);
        }
    }

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    //handle single input
    let spilt_input_by_line = input_line.split('\n').collect::<Vec<&str>>();
    let mut spilt_pattern = ' ';
    //println!("{:?} {pattern}", spilt_input_by_line);

    if spilt_input_by_line.len() > 1 {
        let mut matched: Vec<bool> = Vec::new();
        for v in spilt_input_by_line {
            if match_pattern(v, &pattern) {
                println!("{v}");
                matched.push(true);
            } else {
                matched.push(false)
            }
        }

        if matched.iter().any(|c| *c == true) {
            process::exit(0)
        } else {
            process::exit(1)
        }
    }

    if split_input_by_space.len() <= 1 {
        //println!("{input_line} is a single word ine input");
        if match_pattern(&input_line, &pattern) {
            println!("{input_line}");
            process::exit(0)
        }
    } else {
        //println!("multiword input: {input_line} passed");
        let split_ptn_by_space = pattern.split_whitespace().collect::<Vec<&str>>();
        let res = handle_single_ptn_to_spaced_txt(&split_input_by_space, &split_ptn_by_space);
        println!("{:?}", res);
        if res.len() == split_ptn_by_space.len() {
            println!("{input_line}");
            process::exit(0)
        }
    }

    process::exit(1)
}

fn handle_sentence_ptn(sentences: &Vec<String>, pattern: String) -> Vec<String> {
    //println!("{:?}", sentences);
    let mut final_sentence = String::new();
    let mut sentence_collection: Vec<String> = Vec::new();

    for sentence in sentences.iter() {
        let sentence_spilt: Vec<&str> = sentence.split_whitespace().collect();

        let ptn_split: Vec<&str> = pattern.split_whitespace().collect();
        //println!("{sentence} vs {:?}", ptn_split);
        let res = handle_single_ptn_to_spaced_txt(&sentence_spilt, &ptn_split);

        if res.len() > 0 {
            final_sentence = res
                .into_iter()
                .enumerate()
                .map(|(idx, a)| a.replace("\n", " ").replace("\r", "").replace("\r\n", ""))
                .collect();
            sentence_collection.push(final_sentence);
        }
    }

    //println!("all res: {:?}", sentence_collection);
    return sentence_collection;
}

fn handle_single_ptn_to_spaced_txt(
    split_input_by_space: &Vec<&str>,
    split_ptn_by_space: &Vec<&str>,
) -> Vec<String> {
    let mut res = Vec::new();
    let mut start: usize = 0;
    let input_len = split_input_by_space.len();
    let ptn_len = split_ptn_by_space.len();
    let mut new_ptn_spilt = split_ptn_by_space.to_vec();

    if ptn_len == 1 {
        let ptn = split_ptn_by_space[0];
        for i in (0..input_len - 1) {
            new_ptn_spilt.push(ptn);
        }
    }
    //println!("{:?} vs {:?}", new_ptn_spilt, split_input_by_space);
    while start < split_input_by_space.len() {
        for ptn in new_ptn_spilt.iter() {
            let mut input_to_start_at = &split_input_by_space[start..];
            //println!("{:?}", split_input_by_space);
            for (idx, i) in input_to_start_at.iter().enumerate() {
                //println!("index: {start} matching: {i} == {ptn}");
                let mut i_str = i.to_string();
                let res_str = print_single_matching_line(&i_str, &mut ptn.to_string());
                //println!("resuot: ---{res_str}---{idx}");
                start += 1;
                if res_str.to_string().trim().len() > 0 {
                    if idx == split_input_by_space.len() - 1 {
                        res.push(res_str);
                        //println!("idx reached ==== {start}=== idx {idx}");
                    } else {
                        res.push(format!("{res_str}\n"));
                        break;
                    }
                }
            }
        }
    }

    return res;
}

fn handle_single_matching_line(
    split_input_by_space: &Vec<&str>,
    split_ptn_by_space: Vec<&str>,
) -> Vec<String> {
    let mut res = Vec::new();
    for (i, p) in split_input_by_space.iter().zip(split_ptn_by_space) {
        let mut p_str = p.to_string();

        let res_str = print_single_matching_line(&i.to_string(), &mut p_str);
        //println!(" matching {i} -> {p_str} res-str: {res_str} -o arg");
        if res_str.len() > 0 {
            res.push(res_str)
        }
    }
    //println!("the res string length : {}", res.len());

    return res;
}
