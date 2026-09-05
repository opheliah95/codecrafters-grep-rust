use std::collections::HashMap;
use std::env;
use std::hash::Hash;
use std::io;
use std::process;
use std::vec;

fn start_end_is_pattern(input: &str) -> bool {
    if input.starts_with("^") && input.ends_with("$") {
        return true;
    } else if input.starts_with("(") && input.ends_with(")") {
        return true;
    } else {
        return false;
    }
}
fn remove_start_end(input: &str) -> &str {
    let mut input_chars = input.chars();
    input_chars.next();
    input_chars.next_back();
    let res = input_chars.as_str();

    return res;
}
fn check_all_true(vec: &Vec<bool>) -> bool {
    if vec.len() == 0 {
        return false;
    }
    vec.iter().all(|e| *e == true)
}

fn convert_to_digit(input: &str) -> String {
    let mut res = "".to_string();
    for c in input.chars() {
        if c.is_ascii_digit() {
            res.push_str("//d");

        } else {
            return "".to_string();
        }
    }
    return res

}

fn check_individual_match(input_line: &str, pattern: &str) -> bool {
    println!("FN CHECK_INDV_MATCHING, matching {input_line} -> pattern {pattern}");
    let pattern_clone: Vec<char> = pattern.clone().chars().collect();
    let to_match = ["\\d", "\\w", "\\d+"];
    let mut match_end_pos = 0;
    let mut matches_by_index: HashMap<usize, &str> = HashMap::new();
    for i in to_match.iter() {
        for (idx, matched) in pattern.match_indices(i) {
            matches_by_index
                .entry(idx)
                .and_modify(|longest| {
                    if matched.len() > longest.len() {
                        *longest = matched;
                    }
                })
                .or_insert(matched);
        }
    }

    println!("all patterns to check are {:?}", matches_by_index);

    let mut map_keys: Vec<usize> = matches_by_index.clone().into_keys().collect();
    let mut ptn_to_match = "";

    for (i, c) in input_line.chars().enumerate() {
        //println!("index is {i}, char is {c}, pattern is {pattern}");
        if c == pattern_clone[i] {
            continue;
        } else {
            // start the matching
            if map_keys.contains(&i) {
                println!(
                    "{i} starting matching {}",
                    matches_by_index.get(&i).unwrap()
                );
                ptn_to_match = matches_by_index.get(&i).unwrap();
                match_end_pos = ptn_to_match.len() + i;
                if !match_pattern(&c.to_string(), ptn_to_match) {
                    println!("No recursive pattern at {i} for \"{c}\" to match {ptn_to_match}");
                    return false;
                } else {
                    map_keys.retain(|&x| x != i);
                    println!("at idx {i} {c} contain pattern {ptn_to_match}");
                    continue;
                }
            } else {
                // start to continue patern matching after passing first patch
                println!(
                    "STAGE 3 passing matching idx:  idx {i} \"{c}\" will match {ptn_to_match}"
                );
                if !match_pattern(&c.to_string(), ptn_to_match) {
                    let pattern_final = &pattern[match_end_pos..];
                    println!("continue to match {} {}", pattern_final, &input_line[i..]);
                    check_individual_match(&input_line[i..], pattern_final);
                }
            }
        }
    }

    return true;
}

// currently handles \d \input -> 1 apple
fn pattern_parser(mut input_line: &str, mut pattern: &str) -> bool {
    if start_end_is_pattern(input_line) {
        input_line = remove_start_end(input_line);
    }

    if start_end_is_pattern(pattern) {
        pattern = remove_start_end(pattern);
    }
    println!("------------start parsing {input_line}, PATTERN: {pattern}--------");

    let input_parts: Vec<&str> = input_line.split(" ").collect();
    let pattern_parts: Vec<&str> = pattern.split(" ").collect();
    let mut res: Vec<bool> = Vec::new();

    if input_parts.len() == pattern_parts.len() {
        for (key, val) in input_parts.iter().enumerate() {
            let current_ptn = pattern_parts[key];

            let current_match = match_pattern(val, current_ptn);
            println!(
                "matching word by word: key {} val {} => result is {} ",
                key, val, current_match
            );
            res.push(current_match);
        }

        // for some reason res does not update...
        println!("res {:?} ", res);
        if res.len() == 0 {
            println!("res {:?} ", res);
            return false;
        }

        return res.iter().all(|v| *v == true);
    }

    // if length does not match
    let input_re_spilt: Vec<&str> = input_line.split(" ").collect();
    if pattern.len() < input_line.len() {
        for (idx, p) in pattern_parts.clone().into_iter().enumerate() {
            for (input_idx, mut input_p) in input_re_spilt.clone().into_iter().enumerate() {
                println!("matching now: .... {p} to input {input_p}");
                let input_digits = convert_to_digit(input_p);
                if input_digits.len() != 0 {
                    input_p = &input_digits;
                }
                if match_pattern( input_p, p) {
                    let ptn_slice = &pattern_parts[idx+1..].join("");
                    let input_slice = &input_re_spilt[idx+1..].join("");
                    
                    if match_pattern(input_slice, ptn_slice) {
                        return true;
                    }
                }
                if input_idx == input_re_spilt.len() -1 {
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
    // handle alt
    if pattern.starts_with("(") && pattern.ends_with(")") {
        let match_res = match_pattern(input, pattern);
        res.push(match_res);
        return Some(match_res);
    }

    let mut pt_count: Vec<_> = pattern.match_indices(input_ptn).map(|(i, _)| i).collect();
    println!(
        "pt_count {:?} -- ptn_start {input_ptn} and input {} and pattern {}",
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

fn match_pattern(mut input_line: &str, mut pattern: &str) -> bool {
    match pattern {
        // check has both start and end
        ptn if pattern.starts_with("^") && pattern.ends_with("$") => {
            println!("INNPUT {input_line} start:  ^ and ends: $");
            let pattern_start = pattern.chars().nth(1).unwrap();
            let last_word_pattern_pos = pattern.len() - 2;
            let last_word_pattern = pattern.chars().nth(last_word_pattern_pos).unwrap();
            if !input_line.starts_with(pattern_start) || !input_line.ends_with(last_word_pattern) {
                return false;
            }

            let to_match = &pattern[1..pattern.len() - 1];
            let pattern_lst = ["\\d", "\\w", "+", "d"];
            if pattern_lst.iter().any(|c| to_match.contains(c)) {
                return check_individual_match(input_line, to_match);
            }
            return input_line == to_match;
        }

        // check start anchor
        ptn if pattern.starts_with("^") => {
            let to_match = &pattern[1..pattern.len()];
            return input_line.starts_with(to_match);
        }
        // ch,eck ending
        ptn if pattern.ends_with("$") => {
            let to_match = &pattern[0..pattern.len() - 1];
            return input_line.ends_with(to_match);
        }
        "\\d" => {
            println!("matching digits {input_line}  ----> {pattern}");
            return input_line.chars().any(|e| e.is_ascii_digit())
        },
        "\\d+" => input_line.chars().all(|e| e.is_ascii_digit()),
        "\\d?" => input_line.chars().any(|e| e.is_ascii_digit()) || input_line.len() == 0,
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

        "." => return input_line != ("\\n"),
        "+" => return true,

        //check alt | operator
        ptn if pattern.starts_with("(") && pattern.ends_with(")") => {
            println!("{ptn} eval alternat");
            let ptn_formatted = remove_start_end(ptn);
            let ptn_spilt = ptn_formatted.split("|").collect::<Vec<&str>>();
            if ptn_spilt.len() <= 1 {
                println!("{:?}", ptn_spilt);
                return false;
            } else {
                let mut contain_alt: Vec<bool> = Vec::new();

                for val in ptn_spilt {
                    println!("eval if {input_line} contain {val}");
                    contain_alt.push(input_line.contains(val));
                    println!("the vec is {:?} ", contain_alt);
                }
                return contain_alt.iter().any(|v| *v == true);
            }
        }

        ptn if ptn.starts_with("(") => {
            println!("ptn start with (: {input_line} ----  {ptn}");
            let alt_end = ptn.rfind(")").unwrap_or(0);
            let pipe_find = ptn.find("|").unwrap_or(0);
            if alt_end == 0 || pipe_find == 0 {
                println!("matching  {input_line} -->  {ptn} NO CLOSURE");
                return check_individual_match(input_line, ptn);
            } else if alt_end <= pipe_find {
                println!("NOT CLOSURE!!  {input_line} -->  {ptn} ) appear earlier than |");
                return check_individual_match(input_line, ptn);
            } else {
                // match ()

                let mut ptn_p1_old = ptn.get(1..alt_end).unwrap();
                let ptn_p1_string = format!("({ptn_p1_old})");
                let ptn_p1: &str = &ptn_p1_string;
                println!("( ) | all present => matching {input_line} -------- {ptn_p1}");
                let ptn_1_match = match_pattern(input_line, ptn_p1);
                let ptn_p2 = ptn.get(alt_end + 1..).unwrap();
                if ptn_p2.len() == 0 {
                    return true;
                } else if ptn_1_match {
                    let mut ptn_1_match_start = find_match_inbetween(input_line, ptn_p1_old);
                    println!("input start is {ptn_1_match_start} and will append {ptn_p2}");
                    
                    ptn_1_match_start.push_str(ptn_p2);
                    println!("partial () match, need to match {input_line} => {ptn_1_match_start}");
                    return match_pattern(input_line, ptn_1_match_start.as_str());
                }
                return false;
            }

            return false;
        }

        // check wildcard
        ptn if ptn.contains(".") => {
            for (idx, c) in input_line.chars().enumerate() {
                let wildcard_pos = pattern.find(".").unwrap_or(0);
                let ptn_at_idx = pattern.clone().chars().nth(idx).unwrap_or('\0');
                println!("WILDCARD: idx {idx}, c: {c} -> match ptn: {ptn_at_idx}");

                // if length same then line by line matching
                if input_line.len() == pattern.len() {
                    if !match_pattern(&c.to_string(), &ptn_at_idx.to_string()) {
                        return false;
                    }
                } else {
                    let char_after_wildcard =
                        pattern.clone().chars().nth(wildcard_pos + 1).unwrap();

                    if idx <= wildcard_pos {
                        if !match_pattern(&c.to_string(), &ptn_at_idx.to_string()) {
                            return false;
                        }
                    }

                    // case idx > whildcard pos
                    if vec!['+', '?'].contains(&char_after_wildcard) {
                        // check if end matches -> get parts after +
                        let after_wildcard_slice = &pattern[wildcard_pos + 2..];
                        //reverse search input end to match

                        // terminate if input len actally less than ptn i.e. echo -n 'gol' | ./your_program.sh -E 'g.+gol'
                        let current_input_slice = &input_line[idx..];
                        println!(
                            "WILDCARD: match zero/more quantifier encountered, idx {idx} val in input {c}"
                        );
                        if char_after_wildcard == '+'
                            && after_wildcard_slice.len() >= current_input_slice.len()
                        {
                            return false;
                        }

                        // if char_after_wildcard == '?'{
                        //     return false;
                        // }

                        let after_wildcard_rev: Vec<char> =
                            after_wildcard_slice.chars().into_iter().rev().collect();
                        let mut input_back_rev: Vec<char> =
                            input_line.clone().chars().into_iter().collect();
                        // shorten pattern and re-search wildcard pos
                        for (rev_idx, rev_c) in after_wildcard_rev.iter().enumerate() {
                            let m = input_back_rev.pop().unwrap();
                            println!("Now reverse match at ptn idx {rev_idx} :  {rev_c} -> {m} ");
                            if *rev_c != m {
                                return false;
                            }
                        }
                        return true;
                    } else {
                        return false;
                    }
                }
            }
            return true;
        }

        ptn if pattern.contains("+") || pattern.contains("?") => {
            let mut ptn_quant = "";
            let mut input_quantifier_pos = None;
            if ptn.contains("+") {
                input_quantifier_pos = ptn.find("+");
                ptn_quant = "+";
            } else {
                input_quantifier_pos = ptn.find("?");
                ptn_quant = "?";
            };
            match input_quantifier_pos {
                Some(p) => {
                    let before_p = p - 1;
                    let after_p = p + 1;
                    //handle +
                    let mut letter_to_match = pattern.clone().chars().nth(0);
                    // if ptn_quant == "?" {
                    //     letter_to_match = pattern.clone().chars().nth(before_p - 1); // one letter before prev letter i.e. skip one match
                    // }
                    let letter_after_p = pattern.clone().chars().nth(after_p).unwrap_or('\0');
                    // match start of pattern

                    match letter_to_match {
                        Some(c) => {
                            // fix inputline basing on the first instance of before p
                            let first_letter_to_start = input_line.find(c).unwrap_or(0);
                            let old_input = input_line.clone();
                            input_line = &input_line[first_letter_to_start..];
                            println!(
                                "===QUANT +? MATCHING===starting from {input_line} PREV: {old_input}, search res {c}, res pos {first_letter_to_start}"
                            );

                            // handle case like a+=> apple
                            println!(
                                "TETS START: STEP INIT: checking SINGLE {ptn} match: \"{c}\" {ptn} => {input_line}"
                            );
                            let last_occurance_of_p = pattern.rfind(ptn_quant).unwrap();
                            if before_p == 0 {
                                println!("signle step case passed");
                                return input_line.chars().any(|e| e == c);
                            }

                            // hanndle cases like ca+ts
                            let mut pattern_char = pattern.chars();
                            for (idx, val) in input_line.chars().enumerate() {
                                //println!("enter loop {idx} and val {val}");
                                if idx < before_p {
                                    let pattern_char_before_p =
                                        pattern_char.clone().nth(idx).unwrap();
                                    println!(
                                        "TEST {idx} == step {idx}: checking FULL PTN {ptn} match: VAL {val} => {pattern_char_before_p}"
                                    );
                                    if val != pattern_char_before_p {
                                        return false;
                                    }
                                    // if here idx actuall reaching end
                                    if idx == input_line.len() - 1 {
                                        let ptn_before_quant = ptn.get(idx + 1..p).unwrap();
                                        // if only one letter before ?
                                        println!("ptn before quant: {ptn_before_quant}");
                                        return ptn_quant == "?" && ptn_before_quant.len() == 1;
                                    }
                                }
                                // handle minus sign
                                if idx == before_p {
                                    match ptn_quant {
                                        "?" => {
                                            // does not need to have the previous character
                                            let after_zero_quant =
                                                pattern_char.clone().nth(idx).unwrap();
                                            println!(
                                                "? reached matching {val} to PTN {after_zero_quant}"
                                            );

                                            if val != after_zero_quant {
                                                if val != letter_after_p
                                                {
                                                    println!(
                                                        "{val} does not match ptn {}",
                                                        after_zero_quant
                                                    );
                                                   return false
                                                } 
                                                return true
                                            }
                                            println!(
                                                "@@@MATCHED {val} matched ptn {}@@@",
                                                after_zero_quant
                                            );
                                            // already end of input then we matched fully for cases like dogs -> dogs?
                                            if idx == input_line.len() - 1 {
                                                return true;
                                            }
                                            continue;
                                        }

                                        "+" => {
                                            if val != pattern_char.clone().nth(idx).unwrap() {
                                                return false;
                                            }
                                        }
                                        _ => {
                                            return false;
                                        }
                                    }
                                }

                                // reached plus/? sign -> need to have one match
                                if idx == p {
                                    println!(
                                        "reaching idx == p, value is {val} and ptn after p {} (idx pattern: {after_p}) LTR after p {letter_after_p}",
                                        &pattern[after_p..]
                                    );
                                    // handle ? after ? should only match world by word
                                    if ptn_quant == "?" {
                                        let ptn_to_match = pattern_char.clone().nth(p - 1).unwrap();
                                        println!(
                                            "AT idx == p NOW, check ? ZERP?ONE quantifier:  val {val} == {} ",
                                            ptn_to_match
                                        );
                                        if val == ptn_to_match {
                                            return false;
                                        }
                                    }

                                    if idx == input_line.len() - 1 {
                                        // if pattern also just have one length
                                        if pattern[after_p..].len() == 1 {
                                            return val == letter_after_p;
                                        } else if pattern[after_p..].len() == 0 {
                                            return true; // pattern ends here
                                        } else {
                                            // there is more pattern than input but we only have one idx
                                            if pattern[after_p..].contains("?") {
                                                return pattern[after_p..].ends_with(val); // special case to handle ? match zero
                                            }

                                            return false;
                                        }
                                    }

                                    continue;
                                }

                                if idx > p {
                                    println!(
                                        "step {idx}: WHEN {idx} > {p}: checking match after {ptn}: PATTERN {letter_after_p} => VAL {val} INPUT PASSED {input_line}"
                                    );

                                    letter_to_match = pattern.clone().chars().nth(before_p);
                                    if val != letter_to_match.unwrap() {
                                        // if nothing after +
                                        println!("val is {val} , and to match is {c}");
                                        if letter_after_p == '\0' {
                                            return true;
                                        }

                                        let mut pattern_slice = &pattern[p + 1..];

                                        if last_occurance_of_p != p {
                                            pattern_slice = &pattern[last_occurance_of_p + 1..];
                                        }

                                        println!(
                                            "matching {idx} idx::: {ptn} pattern: PTN {} VAL {}",
                                            pattern_slice,
                                            &input_line[idx - 1..]
                                        );

                                        return input_line[idx - 1..].starts_with(pattern_slice);
                                    }
                                    continue;
                                }
                            }
                        }
                        None => {
                            println!("no char found");
                            return false;
                        }
                    }
                }
                None => return false,
            }
            //println!("the string passed is: {}", input_line);
            return false;
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
    let split_input = input_line.split(" ").collect::<Vec<&str>>().len();
    if split_input == 1 {
        println!("{input_line} is a single word ine input");
        if match_pattern(&input_line, &pattern) {
            println!("single input pattern {input_line} passed");
            process::exit(0)
        } else {
            process::exit(1)
        }
    } else {
        println!("multiword input: {input_line} passed");
        if pattern_parser(&input_line, &pattern) {
            println!("matching results achieved!");
            process::exit(0)
        } else {
            process::exit(1)
        }
    }
}
