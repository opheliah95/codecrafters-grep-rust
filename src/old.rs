fn pattern_parser(input_line: &str, pattern: &str) -> bool {
    println!("------------start parsing {input_line}, PATTERN: {pattern}--------");
    let input_parts: Vec<&str> = input_line.splitn(2, " ").collect();
    let pattern_parts: Vec<&str> = pattern.splitn(2, " ").collect();

    if input_parts.len() == pattern_parts.len() && input_parts.len() == 2 {
        let (input_1, input_2) = (input_parts[0], input_parts[1]);
        let (pattern_1, pattern_2) = (pattern_parts[0], pattern_parts[1]);
        println!("input spilt: {:?} pattern spilt {:?}", input_parts, pattern_parts);
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


//this function works with \d\apple \d\d\d \w\w\ws etc
fn check_input_pattern(
    input: &str,
    pattern: &str,
    input_ptn: &str,
) -> bool {
    // base case input == pattern
    if input.contains(pattern) {
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