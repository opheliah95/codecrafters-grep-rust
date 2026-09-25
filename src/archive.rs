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


pub fn examine_repeat(mut input_line: &str, mut pattern: &str) -> Option<String> {
    // handle plural cases
    if pattern.ends_with("s") && !input_line.ends_with("s") {
        eprintln!("plural not matching");
        return None;
    }

    let mut ptn = pattern.clone().to_string();

    let contain_quant = check_quant_pattern(ptn);

  

    let mut repeats = vec!["\\w", "\\d", "\\d+", "\\w+"];
    let mut input_filtered = String::new();

    if input_line.contains(pattern) && !repeats.contains(&pattern) {
        let matches: Vec<_> = input_line.match_indices(pattern).collect();
        let matches_len = matches.len();
        if matches_len == 0 {
            return None;
        } else {
            for (idx, m) in matches {
                if idx == matches_len - 1 {
                    input_filtered.push_str(&m.to_string());
                } else {
                    input_filtered.push_str(&format!("{m}\n"));
                }
            }
            return Some(input_filtered);
        }
    }

    let mut input_temp = input_line.replace(",", "");

    let mut repeats_result: HashMap<String, usize> = HashMap::new();

    for repeat in repeats.into_iter() {
        if pattern.contains(repeat) {
            let count = pattern.matches(repeat).count();
            if count > 0 {
                repeats_result.insert(repeat.to_string(), count);
            }
        }
    }

    if repeats_result.is_empty() {
        return None;
    }
    eprintln!(
        "_FN_examine_pattern {input_line} -> examine_repeat: {pattern} {:?}",
        repeats_result
    );
    for (repeat, count) in &repeats_result {
        if ["\\w", "\\w+"].contains(&repeat.as_str()) {
            let input_filtered = input_temp
                .chars()
                .filter(|a| a.is_alphanumeric() || *a == '_')
                .collect::<String>();
            input_temp = input_filtered;
        }

        let repeat_len = repeat.len();
        let input_line_end = input_temp.len();
        let mut diff = "";

        if input_line_end > *count {
            diff = &input_temp[*count..];
        }

        // eprintln!(
        //     "{input_temp} -> {repeat} -> {count} -> diff len {diff} -> {}",
        //     *count + diff.len()
        // );

        if input_line_end == *count || repeat_len * count == pattern.len() {
            eprintln!(
                "MATCHING REPEAT: {input_temp} -> {repeat} with {}",
                repeat_len * count
            );
            format_input_of_repeated_char_pattern(
                &input_temp,
                &mut input_filtered,
                repeat,
                input_line_end,
                *count,
            );
        } else if input_line_end == *count + diff.len() {
            eprintln!("{input_temp} vs {pattern} and diff is {diff}");
            if pattern.ends_with(diff) {
                format_input_of_repeated_char_pattern(
                    &input_temp,
                    &mut input_filtered,
                    repeat,
                    input_line_end,
                    *count,
                );
            }
        } else {
            return None;
        }
    }
    //println!("res is {:?}", res_output);
    //input_filtered.push_str(diff);
    if input_filtered.len() > 0 {
        return Some(input_filtered);
    } else {
        return None;
    }
}