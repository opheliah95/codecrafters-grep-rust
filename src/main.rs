use std::env;
use std::io::{self, Read};
use std::process;
mod lib;
use lib::{check_digits, exit_process_errored, remove_start_end, start_end_is_pattern};
mod match_func;
use match_func::{match_pattern, print_single_matching_line};

fn match_digits(input: &str, pattern: &str) -> bool {
    // if just a string of no
    if input == pattern {
        return true;
    }

    let digit_len = input.len();
    let mut ptn_digit_match: Vec<_> = pattern
        .match_indices("\\d")
        .into_iter()
        .map(|(i, c)| (i, c.to_string()))
        .collect();
    println!(
        "check {input} and its re-matching ptn: {:?}",
        ptn_digit_match
    );

    if ptn_digit_match.len() == 0 {
        return false;
    }

    let start = ptn_digit_match[0].0;
    let digit_match_size = ptn_digit_match.len();
    let mut end = start;
    if digit_match_size > 1 {
        end = ptn_digit_match[digit_match_size - 1].0
    }
    for (idx, c) in pattern.chars().enumerate() {
        if c != '\\' && c != 'd' {
            let c_str: String = c.to_string();
            if idx < start || idx < end {
                if idx <= digit_match_size {
                    ptn_digit_match.insert(idx, (idx, c_str.clone()));
                } else {
                    ptn_digit_match.push((idx, c_str.clone()));
                }
            }
            if idx >= end {
                ptn_digit_match.push((idx, c_str.clone()));
            }
        }
    }
    if ptn_digit_match.len() != input.len() {
        return false;
    } else {
        for (idx, val) in input.chars().enumerate() {
            let input_match = &val.to_string();
            let ptn_match = &ptn_digit_match[idx].1;
            if !match_pattern(input_match, ptn_match) {
                return false;
            }
        }
        return true;
    }
    println!("the start is {start} and content is {:?}", ptn_digit_match);
    println!("matched ptns {:?}", ptn_digit_match);
    return false;
}

// fn digit string spilter

// currently handles \d \input -> 1 apple
fn pattern_parser(mut input_line: &str, mut pattern: &str, spilt_regex: char) -> bool {
    if start_end_is_pattern(input_line) {
        input_line = remove_start_end(input_line);
    }

    if start_end_is_pattern(pattern) {
        pattern = remove_start_end(pattern);
    }
    //println!("------------start parsing {input_line}, PATTERN: {pattern}--------");

    let input_parts: Vec<&str> = input_line.split(spilt_regex).collect();
    let pattern_parts: Vec<&str> = pattern.split(spilt_regex).collect();
    let mut res: Vec<bool> = Vec::new();

    if input_parts.len() == pattern_parts.len() {
        for (key, val) in input_parts.iter().enumerate() {
            let current_ptn = pattern_parts[key];

            let current_match = match_pattern(val, current_ptn);
            // println!(
            //     "matching word by word: key {} val {} => result is {} ",
            //     key, val, current_match
            // );
            res.push(current_match);
        }

        // for some reason res does not update...
        //println!("res {:?} ", res);
        if res.len() == 0 {
            //println!("res {:?} ", res);
            return false;
        }

        return res.iter().all(|v| *v == true);
    }

    // if length does not match
    let mut input_re_spilt: Vec<&str> = input_line.split(" ").collect();
    let mut found = 0;
    let input_end = input_re_spilt.len() - 1;
    if pattern.len() < input_line.len() {
        for (idx, p) in pattern_parts.clone().into_iter().enumerate() {
            input_re_spilt = input_re_spilt[found..].to_vec();
            for (input_idx, mut input_p) in input_re_spilt.clone().into_iter().enumerate() {
                let input_digits = check_digits(input_p);
                println!(
                    "matching PTN to each input iter: ITER {input_idx}: .... {p} to input {input_p}"
                );
                if match_pattern(input_p, p) {
                    let ptn_slice = &pattern_parts[idx + 1..].join("");
                    let input_slice = &input_re_spilt[input_idx + 1..].join("");
                    found = input_idx;
                    println!(
                        "matching P2 PTN_SLICE: .... {ptn_slice} to input_SLICE {input_slice}"
                    );
                    if match_pattern(input_slice, ptn_slice) {
                        return true;
                    }
                    break;
                } else if input_digits > 0 && p.contains("\\d") {
                    println!("input {input_p} is a digit with {input_digits} digits");
                    if match_digits(input_p, p) {
                        if input_idx == input_re_spilt.len() - 1 {
                            return true;
                        }
                        found = input_idx;
                        println!("==breaking out of loop {input_p} and PTN {p} matched==");
                        break;
                    }
                    // temp solution to handle /w/w/w pattern
                } else if p.contains("\\w") {
                    println!("word check for {input_p} -> PTN {p} ");
                    if check_input_pattern(input_p, p, "\\w") {
                        if input_idx == input_re_spilt.len() - 1 {
                            return true;
                        }
                        found = input_idx;
                        break;
                    }
                }
                if input_idx == input_re_spilt.len() - 1 {
                    return false;
                }
            }
        }

        return false;
    }
    return false;
}

// helper function --word by word match
fn find_match_inbetween(source: &str, pattern: &str) -> String {
    if source.len() == 0 || pattern.len() == 0 {
        return "".to_string();
    }

    println!("matching word by word source {source} and ptn: {pattern}");
    let mut start_pos: usize = 0;
    //let mut end_pos:usize = 0;
    let mut matched_chars: Vec<char> = Vec::new();
    for (s_idx, s_val) in source.chars().enumerate() {
        for (p_idx, p_val) in pattern.chars().enumerate() {
            //println!("s_val {s_val}, start pos : {start_pos}  and p_val {p_val} and s_idx {s_idx}");
            if s_val == p_val {
                if start_pos > 0 && start_pos != s_idx - 1 {
                    matched_chars = Vec::new();
                }

                start_pos = s_idx;
                matched_chars.push(s_val);
            }
        }
    }

    let res = matched_chars.iter().collect();
    return res;
}

//this function works with \d\apple \d\d\d \w\w\ws etc
fn check_input_pattern(input: &str, pattern: &str, input_ptn: &str) -> bool {
    // base case input == pattern
    if input.contains(pattern) {
        return true;
    }
    // handle plural
    if pattern.ends_with("s") && !input.ends_with("s") {
        return false;
    }
    // handle alt
    if pattern.starts_with("(") && pattern.ends_with(")") {
        let match_res = match_pattern(input, pattern);
        return match_res;
    }

    let mut pt_count: Vec<_> = pattern.match_indices(input_ptn).map(|(i, _)| i).collect();
    println!(
        "pt_count {:?} -- ptn_start {input_ptn} and input {} and pattern {}",
        pt_count, input, pattern
    );

    if pt_count.len() == 0 {
        return false;
    }

    if pt_count.len() == 1 {
        let matched = match_pattern(input, pattern);
        println!("{input} matched one pattern");
        return true;
    } else {
        //println!("more than one pattern {} {:?} {}", pattern, pt_count, input);
        let pt_count_len = pt_count.len();
        let mut res: Vec<bool> = Vec::new();
        if pt_count_len == input.len() {
            println!("matching...{}", input);
            for i in [0..input.len()] {
                let matched = match_pattern(&input[i], input_ptn);
                res.push(matched);
            }
        } else if pt_count_len < input.len() {
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
            } else {
                println!(
                    "last pt: {last_ptn_pos}, final_ptn:  {final_ptn}, input_not_matched: {input_not_matched}"
                );
                for i in [0..matchable_input.len()] {
                    let matched = match_pattern(&matchable_input[i], input_ptn);
                    res.push(matched);
                }
            }
        }
        return res.into_iter().any(|c| c == true);
    }
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");
    let mut input_line = String::new();
    io::stdin().read_to_string(&mut input_line).unwrap();
    let mut pattern = env::args().nth(2).unwrap();
    let split_input_by_space = input_line.split_whitespace().collect::<Vec<&str>>();

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

            //println!("{:?} vs PTN {:?}", split_input_by_space, split_ptn_by_space);

            if split_ptn_by_space.len() == split_input_by_space.len() {
                res = handle_single_matching_line(&split_input_by_space, split_ptn_by_space);
            } else if split_ptn_by_space.len() != split_input_by_space.len() {
                res = handle_single_ptn_to_spaced_txt(&split_input_by_space, split_ptn_by_space);
            } else if split_ptn_by_space.len() == 0 || split_ptn_by_space.len() == 0 {
                exit_process_errored();
            }
        }

        if res.len() >= 1 {
            println!("{}", res.join(""));
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
        if pattern_parser(&input_line, &pattern, spilt_pattern) {
            println!("{input_line}");
            process::exit(0)
        }
    }

    process::exit(1)
}

fn handle_sentence_ptn(sentences: &Vec<String>, pattern: String) -> Vec<String> {
    let mut final_sentence = String::new();
    let mut sentence_collection: Vec<String> = Vec::new();
    for sentence in sentences.iter() {
        let sentence_spilt: Vec<&str> = sentence.split_whitespace().collect();
        let ptn_split: Vec<&str> = pattern.split_whitespace().collect();
        let res = handle_single_ptn_to_spaced_txt(&sentence_spilt, ptn_split);
        let res_len = res.len();
        final_sentence = res
            .into_iter()
            .enumerate()
            .map(|(idx, a)| a.replace("\n", " ").replace("\r", "").replace("\r\n", ""))
            .collect();
        sentence_collection.push(final_sentence);
    }

    //println!("all res: {:?}", sentence_collection);
    return sentence_collection;
}

fn handle_single_ptn_to_spaced_txt(
    split_input_by_space: &Vec<&str>,
    split_ptn_by_space: Vec<&str>,
) -> Vec<String> {
    let mut res = Vec::new();
    let mut start: usize = 0;
    //println!("spilt input by space {:?}", split_input_by_space);
    for ptn in split_ptn_by_space.iter() {
        let mut input_to_start_at = &split_input_by_space[start..];
        for (idx, i) in input_to_start_at.iter().enumerate() {
            //println!("index: {start} matching: {i} vs {ptn}");
            let mut i_str = i.to_string();
            let res_str = print_single_matching_line(&i_str, &mut ptn.to_string());
            //println!("resuot: ---{res_str}---");
            if res_str.len() > 0 {
                if idx == split_input_by_space.len() - 1 {
                    res.push(res_str);
                    //println!("idx reached ==== {start}=== idx {idx}");
                } else {
                    res.push(format!("{res_str}\n"));
                    start = idx + 1;
                    break;
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
