use std::process;

pub fn check_all_true(vec: &Vec<bool>) -> bool {
    if vec.len() == 0 {
        return false;
    }
    vec.iter().all(|e| *e == true)
}

pub fn contain_digits(input: &str) -> bool {
    for c in input.chars() {
        if c.is_ascii_digit() {
            return true;
        }
    }
    return false;
}

// helper function --word by word match
pub fn find_match_inbetween(source: &str, pattern: &str) -> String {
    if source.len() == 0 || pattern.len() == 0 {
        return "".to_string();
    }

    //println!("matching word by word source {source} and ptn: {pattern}");
    let mut start_pos: usize = 0;
    //let mut end_pos:usize = 0;
    let source_len = source.len();
    let pattern_len = pattern.len();
    let mut matched_chars: Vec<char> = Vec::new();

    // remove all brackets
    let special_symbols = ["|", ")", "(", "$", "^", "?"];
    let source_contain_symbols = special_symbols.iter().any(|a| source.contains(*a));
    let mut new_pattern = pattern;
    if !source_contain_symbols {
        for s in special_symbols.iter() {
            pattern.replace(s, "");
        }
    }

    //println!("pattern is now {pattern}");

    for (s_idx, s_val) in source.chars().enumerate() {
        for (p_idx, p_val) in pattern.chars().enumerate() {
            //println!("s_val {s_val}, start pos : {start_pos}  and p_val {p_val} and s_idx {s_idx}");
            if s_val == p_val {
                if start_pos == 0 {
                    start_pos = s_idx;
                } else {
                    start_pos += 1;
                }
                matched_chars.push(s_val);
                break;
            }

            if start_pos >= source_len - 1 {
                println!("{:?}", matched_chars);
                return matched_chars.iter().collect();
            }
        }
    }
    let res = matched_chars.iter().collect();
    return res;
}

pub fn remove_start_end(input: &str) -> &str {
    let mut input_chars = input.chars();
    input_chars.next();
    input_chars.next_back();
    let res = input_chars.as_str();

    return res;
}

pub fn start_end_is_pattern(input: &str) -> bool {
    if input.starts_with("^") && input.ends_with("$") {
        return true;
    } else if input.starts_with("(") && input.ends_with(")") {
        return true;
    } else {
        return false;
    }
}

pub fn check_digits(input: &str) -> usize {
    let mut res = 0;
    for c in input.chars() {
        if c.is_ascii_digit() {
            res += 1;
        } else {
            return 0;
        }
    }
    return res;
}

pub fn exit_process_errored(is_last: bool, match_found: &mut Vec<bool>) -> bool {
    if is_last && match_found.is_empty() {
        eprintln!("At last index...ERROR===Exit");
        process::exit(1);
    }
    eprintln!("<<NO Match>> -> MOVE to next file...No match...");
    return is_last;
}

pub fn can_success_exit(is_last: bool, match_found: &mut Vec<bool>) -> bool {
    match_found.push(true);
    eprintln!("========MATCH FOUND=====");
    if is_last {
        eprintln!("======Exit Success==========");
        process::exit(0);
    }
    return is_last;
}
