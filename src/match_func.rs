use crate::lib::{find_match_inbetween, remove_start_end};
use std::collections::HashMap;

pub fn examine_repeat(mut input_line: &str, mut pattern: &str) -> Option<String> {
    let repeats = ["\\w", "\\d"];
    let mut input_temp = &input_line.replace(",", "");
    let mut res_output: Vec<String> = vec![];

    let mut repeats_result: HashMap<String, usize> = HashMap::new();
    for repeat in repeats.into_iter() {
        if pattern.contains(repeat) {
            let res = repeats_result.entry(repeat.to_string()).or_insert(1);
            *res += 1;
        }
    }

    //println!("examine_repeat");
    for (repeat, count) in &repeats_result {
        let repeat_len = repeat.len();
        //println!("{input_temp} -> {repeat}");
        if input_temp.len() == repeat_len {
            //println!("SUCCES: {input_temp} -> {repeat}");
            let input_filtered: String = input_temp
                .chars()
                .into_iter()
                .filter(|a| match_pattern(&a.to_string(), repeat))
                .collect();

            //println!("{input_filtered}");
            return Some(input_filtered);
        } else {
            return None;
        }
    }
    //println!("res is {:?}", res_output);

    return None;
}

pub fn match_pattern(mut input_line: &str, mut pattern: &str) -> bool {
    match pattern {
        // check has both start and end
        ptn if pattern.starts_with("^") && pattern.ends_with("$") => {
            //println!("INNPUT {input_line} start:  ^ and ends: $");
            let to_match = &pattern[1..pattern.len() - 1];
            let ptn_spilt = to_match.split(" ").collect::<Vec<&str>>();

            if ptn_spilt.len() > 1 {
                let input_split = input_line.split(" ").collect::<Vec<&str>>();
                if input_split.len() == 0 || input_split.len() != ptn_spilt.len() {
                    return false;
                }

                for (idx, v) in input_split.iter().enumerate() {
                    let current_ptn = ptn_spilt[idx];
                    if !match_pattern(v, current_ptn) {
                        return false;
                    }
                }
                //println!("{input_line}");
                return true; // if all matched within start/end
            }

            let pattern_start = pattern.chars().nth(1).unwrap();
            let last_word_pattern_pos = pattern.len() - 2;
            let last_word_pattern = pattern.chars().nth(last_word_pattern_pos).unwrap();
            //println!("last wprd {last_word_pattern}");
            if !input_line.starts_with(pattern_start) || !input_line.ends_with(last_word_pattern) {
                return false;
            }
            //println!("input is now {input_line}");

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
            let regex: Vec<&str> = vec!["\\d", "\\w", "(", ")"];
            if regex.iter().any(|a| to_match.contains(a)) {
                let mut new_input = &input_line[0..input_line.len() - 1];
                let res = match_pattern(&new_input, to_match);
                //println!("matching {input_line} to {pattern} and res is {res}");
                return res;
            } else {
                return input_line.ends_with(to_match);
            }
        }
        "\\d" => {
            //println!("matching digits {input_line}  ----> {pattern}");
            return input_line.chars().any(|e| e.is_ascii_digit());
        }
        "\\d+" => input_line.chars().all(|e| e.is_ascii_digit()),
        "\\d?" => input_line.chars().any(|e| e.is_ascii_digit()) || input_line.len() == 0,
        "d" => input_line.starts_with(|s: char| s.is_ascii_alphabetic()),
        "\\w" => {
            input_line
                .chars()
                .any(|e| e.is_ascii_alphanumeric() || e == '_')
                && input_line.len() == 1
        }
        "\\w+" => return input_line.len() >= 1 && match_pattern(input_line, "\\w"),
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
            //println!("{ptn} eval alternat");
            let ptn_formatted = remove_start_end(ptn);
            let ptn_spilt = ptn_formatted.split("|").collect::<Vec<&str>>();
            if ptn_spilt.len() <= 1 {
                println!("{:?}", ptn_spilt);
                return false;
            } else {
                let mut contain_alt: Vec<bool> = Vec::new();

                for val in ptn_spilt {
                    //println!("eval if {input_line} contain {val}");
                    contain_alt.push(input_line.contains(val));
                    //println!("the vec is {:?} ", contain_alt);
                }
                return contain_alt.iter().any(|v| *v == true);
            }
        }

        ptn if ptn.starts_with("(") => {
            //println!("ptn start with (: {input_line} ----  {ptn}");
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
                //println!("( ) | all present => matching {input_line} -------- {ptn_p1}");
                let ptn_1_match = match_pattern(input_line, ptn_p1);
                let ptn_p2 = ptn.get(alt_end + 1..).unwrap();
                if ptn_p2.len() == 0 {
                    return true;
                } else if ptn_1_match {
                    let mut ptn_1_match_start = find_match_inbetween(input_line, ptn_p1_old);
                    //println!("input start is {ptn_1_match_start} and will append {ptn_p2}");

                    ptn_1_match_start.push_str(ptn_p2);
                    // println!("partial () match, need to match {input_line} => {ptn_1_match_start}");
                    return match_pattern(input_line, ptn_1_match_start.as_str());
                }
                return false;
            }
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
                            // println!(
                            //     "===QUANT +? MATCHING===starting from {input_line} PREV: {old_input}, search res {c}, res pos {first_letter_to_start}"
                            // );

                            // // handle case like a+=> apple
                            // println!(
                            //     "TETS START: STEP INIT: checking SINGLE {ptn} match: \"{c}\" {ptn} => {input_line}"
                            // );
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
                                    // println!(
                                    //     "TEST {idx} == step {idx}: checking FULL PTN {ptn} match: VAL {val} => {pattern_char_before_p}"
                                    // );
                                    if val != pattern_char_before_p {
                                        return false;
                                    }
                                    // if here idx actuall reaching end
                                    if idx == input_line.len() - 1 {
                                        let ptn_before_quant = ptn.get(idx + 1..p).unwrap();
                                        // if only one letter before ?
                                        //println!("ptn before quant: {ptn_before_quant} and quant is {ptn_quant}");
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
                                            // println!(
                                            //     "? reached matching {val} to PTN {after_zero_quant}"
                                            // );

                                            if val != after_zero_quant {
                                                if val != letter_after_p {
                                                    println!(
                                                        "{val} does not match ptn {}",
                                                        after_zero_quant
                                                    );
                                                    return false;
                                                }
                                                return true;
                                            }
                                            // println!(
                                            //     "@@@MATCHED {val} matched ptn {}@@@",
                                            //     after_zero_quant
                                            // );
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
                                    // println!(
                                    //     "reaching idx == p, value is {val} and ptn after p {} (idx pattern: {after_p}) LTR after p {letter_after_p}",
                                    //     &pattern[after_p..]
                                    // );
                                    // handle ? after ? should only match world by word
                                    if ptn_quant == "?" {
                                        let ptn_to_match = pattern_char.clone().nth(p - 1).unwrap();
                                        // println!(
                                        //     "AT idx == p NOW, check ? ZERP?ONE quantifier:  val {val} == {} ",
                                        //     ptn_to_match
                                        // );
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

pub fn check_individual_match(input_line: &str, pattern: &str) -> bool {
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

pub fn remove_underline_and_punc(input_line: &String) -> String {
    let mut new_input = input_line.replace("_", " ");
    new_input = new_input.replace(",", "");
    new_input = new_input.replace(".", "");
    return new_input;
}

pub fn print_single_matching_line(input_line: &String, pattern: &mut String) -> String {
    let new_input = remove_underline_and_punc(&input_line);
    let mut input_slice = new_input.split_whitespace().collect::<Vec<&str>>();
    //println!("{new_input} **{pattern}**");
    let input_slice_len = input_slice.len();
    let mut res: Vec<char> = Vec::new();
    let mut res_str: Vec<String> = vec![];

    // simplest case exact match
    if input_line == pattern {
        return input_line.to_string();
    }


    for (idx, val) in input_slice.into_iter().enumerate() {
        if let Some(repeat_matched) = examine_repeat(val, pattern) {
            res_str.push(repeat_matched);
            
            if idx == input_slice_len - 1 {
                let output_len = res_str.len();
                let out = format_matched_vec(&res_str, output_len);
                return out
            }
            continue;
        }

        if match_pattern(val, pattern) {
            //println!("==={val} MATCHED {pattern}===");
            if pattern == "\\d" {
                for v_char in val.chars().into_iter() {
                    //println!("{v_char} is a digit...");
                    if let Some(v) = digit_count_and_return(v_char) {
                        res.push(v);
                    }
                }

                if res.len() == 0 {
                    return "".to_string();
                } else if idx == input_slice_len - 1 {
                    let output = res
                        .iter()
                        .enumerate()
                        .map(|(idx, a)| {
                            if idx < res.len() - 1 {
                                format!("{a}\n")
                            } else {
                                format!("{a}")
                            }
                        })
                        .collect();

                    return output;
                }
            } else if pattern == "\\d+" {
                let digit = val.chars().find(|c| c.is_ascii_digit());
                match (digit) {
                    Some(d) => {
                        let idx = val.find(d).unwrap();
                        let mut res: Vec<char> = Vec::new();
                        res.push(d);
                        let mut next_idx = idx + 1;
                        if val.len() == idx + 1 {
                            return d.to_string();
                        }

                        while next_idx < val.len() {
                            let next_char = val.chars().nth(next_idx).unwrap();

                            if next_char.is_ascii_digit() {
                                res.push(next_char);
                            } else {
                                break;
                            }
                            next_idx += 1;
                        }

                        return res.into_iter().collect();
                    }
                    None => return "".to_string(),
                }
            } else {
                if pattern.starts_with("^") {
                    *pattern = pattern[1..].to_string();
                }

                if pattern.ends_with("$") {
                    *pattern = pattern[..pattern.len() - 1].to_string();
                }

                if pattern.contains("(") && pattern.contains(')') {
                    res_str.push(val.to_string());
                    //println!("res_st: {:?}  {input_slice_len}", res_str);
                    if idx == input_slice_len - 1 {
                        if res_str.len() == 0 {
                            return "".to_string();
                        } else {
                            //println!("done");
                            let output_len = res_str.len();
                            let out = format_matched_vec(&res_str, output_len);

                            return out;
                        }
                    }

                    continue;
                }

                let items: Vec<char> = val
                    .chars()
                    .zip(pattern.chars())
                    .filter(|(x, y)| x == y || *y == '?')
                    .map(|(x, _)| x)
                    .collect();

                //intln!("items are {:?}", items);
                if !items.is_empty() {
                    let result: String = items.into_iter().collect();
                    return result;
                } else {
                    return "".to_string();
                }
            }
        } else {
            return "".to_string();
        }
    }
    return "".to_string();
}

fn format_matched_vec(res_str: &Vec<String>, output_len: usize) -> String {
    let out: String = res_str
        .into_iter()
        .enumerate()
        .map(|(idx, a)| {
            if idx != output_len - 1 {
                format!("{a}\n")
            } else {
                format!("{a}")
            }
        })
        .collect();
    out
}

fn digit_count_and_return(val: char) -> Option<char> {
    let digit = val.is_ascii_digit();
    match (digit) {
        true => {
            //println!("the digit is {val}");
            return Some(val);
        }

        false => {
            return None;
        }
    }
}
