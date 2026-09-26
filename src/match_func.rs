use crate::lib::{find_index_of_ptn, find_match_inbetween, remove_start_end};
use std::{collections::HashMap, f32::consts::E};

pub fn count_single_repeat(mut pattern: &str) -> usize {
    // handle plural cases

    let mut repeats = vec!["\\w", "\\d", "\\d+", "\\w+"];

    for repeat in repeats.into_iter() {
        if pattern.contains(repeat) {
            let count = pattern.matches(repeat).count();
            if count * repeat.len() == pattern.len() {
                return count;
            }
        }
    }
    return 0;
}

pub fn count_ptn_len(mut pattern_split: Vec<String>) -> usize {
    let mut ptn_len = pattern_split.len();
    //println!("ptn len {ptn_len} and {:?}", pattern_split);
    for ptn in pattern_split {
        if ptn.ends_with("?") && !ptn.starts_with("(") {
            //println!("found it {ptn}");
            ptn_len -= 1;
        }
    }

    if ptn_len <= 1 {
        return 1;
    } else {
        return ptn_len;
    }
}

pub fn re_formatted_res_with_pattern(input: Vec<&str>, pattern: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut c = 0;
    let count = count_single_repeat(pattern);

    eprintln!(
        "Fn==re_formatted_res_with_pattern==the input is {:?} and count is {}",
        input, count
    );
    if count <= 1 {
        return input.iter().map(|a| a.to_string()).collect();
    }
    while c < input.len() {
        if input[c].len() == count {
            out.push(input[c].to_string());
            c += 1;
        } else if c + count <= input.len() {
            // Safely slice count elements when available
            out.push(input[c..c + count].concat());
            c += count;
        } else {
            // Push remaining trailing elements individually or as a final slice
            out.push(input[c..].concat());
            break;
        }
    }

    out
}

pub fn check_quant_pattern(mut ptn: String) -> Option<(usize, usize, String)> {
    let quant_start = find_index_of_ptn(ptn.as_str(), "{");
    let quant_end = find_index_of_ptn(ptn.as_str(), "}");

    match (quant_start, quant_end) {
        (Some(start), Some(end)) => {
            if start >= end {
                return None;
            } else {
                let quant_words = &ptn[start + 1..end];

                match quant_words.parse::<usize>() {
                    Ok(n) => {
                        eprintln!(
                            "___FN___check_quant_pattern-> checking quantifiers...break down input {:?}",
                            ptn.chars()
                        );
                        let mut letter_before = ptn.chars().nth(start - 1).unwrap().to_string();

                        if vec!["]", ")"].contains(&letter_before.as_str()) {
                            let bracket_start = match &*letter_before {
                                "]" => "[",
                                _ => "(",
                            };
                            let bracket_start_idx = find_index_of_ptn(&ptn, bracket_start);
                            match bracket_start_idx {
                                Some(start_idx) => {
                                    let range_between = &ptn[start_idx + 1..start - 1];
                                    let range_before = &ptn[0..start_idx];
                                    let range_full = &ptn[start_idx..start].repeat(n);
                                    let ptn_formatted = format!("{range_before}{range_full}");
                                    eprintln!("bracket start idx: {range_before} vs {range_between} vs fornmatted={ptn_formatted}");

                                    return Some((start, end, ptn_formatted.to_string()));
                                }

                                None => {}
                            }
                        };
                        // handling single letters
                        let mut quant_start = start;
                        let slash = &ptn[start - 2..start - 1];
                        if slash == "\\" {
                            letter_before = format!("\\{letter_before}");
                            quant_start -= 1;
                        };

                        eprintln!(
                            "___FN___check_quant_pattern-> letter to repeat: {letter_before} for {n} times"
                        );
                        let repeated_letter = letter_before.repeat(n);
                        // need to remove the letter to repeat
                        let ptn_new = format!(
                            "{}{}{}",
                            &ptn[0..quant_start - 1],
                            repeated_letter,
                            &ptn[end + 1..]
                        );
                        eprintln!(
                            "___FN___check_quant_pattern->Quant MATCH the new pattern is {ptn_new} -> matching {n}"
                        );

                        return Some((start, end, ptn_new));
                    }
                    Err(e) => {
                        eprintln!("___FN___check_quant_pattern->{quant_words} is not digits");
                        return None;
                    }
                };
            }
        }
        _ => {
            return None;
        }
    }
}

pub fn examine_repeat(mut input_line: &str, mut pattern: &str) -> Option<String> {
    // handle plural cases
    if pattern.ends_with("s") && !input_line.ends_with("s") {
        eprintln!("plural not matching");
        return None;
    }

    let mut repeats = vec!["\\w", "\\d", "\\d+", "\\w+"];
    let mut input_filtered = String::new();

    if input_line.contains(pattern) && !repeats.contains(&pattern) {
        let matches: Vec<_> = input_line.match_indices(&pattern).collect();
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
        let mut count_end: &str = "";
        let mut pattern_diff_end = "";

        if input_line_end > *count {
            diff = &input_temp[*count..];
            let count_end = pattern.rfind(repeat);
            match count_end {
                Some(n) => {
                    eprintln!("repeat ends at {n} => {}", pattern.chars().nth(n).unwrap());
                    pattern_diff_end = &pattern[n + repeat_len..];
                }
                None => {
                    // meaning ptn end with ptn;
                    //eprintln!("PTN ends without anything else...");
                }
            }
        }

        eprintln!(
            "__FN__examine_repeat__INPUT={input_temp} ->REPEAT={repeat} ->REPEAT_COUNT={count} -> diff len {diff} -> {}",
            *count + diff.len()
        );

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
            // expanded make sure not count /d into matching only match non-patterns
            if diff.ends_with(pattern_diff_end) {
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

fn format_input_of_repeated_char_pattern(
    input_temp: &String,
    input_filtered: &mut String,
    repeat: &String,
    input_line_end: usize,
    count: usize,
) {
    eprintln!("format input: {input_temp}");
    let mut input_res = input_temp
        .chars()
        .into_iter()
        .filter(|a| match_pattern(&a.to_string(), repeat))
        .collect::<Vec<char>>();

    let input_res_len = input_res.len();
    if input_res_len == 0 {
        *input_filtered = "".to_string();
        return;
    }

    match (input_res_len % count) {
        0 => {}
        quotient => {
            input_res = input_res[0..input_res_len - quotient].to_vec();
        }
    }

    *input_filtered = input_res
        .into_iter()
        .enumerate()
        .map(|(idx, a)| {
            if count == input_res_len || ["\\w+", "\\d+"].contains(&repeat.as_str()) {
                a.to_string()
            } else {
                format!("{a}\n")
            }
        })
        .filter(|a| !a.is_empty() && a != "\n")
        .collect();
}

pub fn match_pattern(mut input_line: &str, mut pattern: &str) -> bool {
    eprintln!("===FN match_pattern -> Matching: {input_line} to {pattern}");
    if pattern.ends_with("s") && !input_line.ends_with("s") {
        eprintln!("plural not matching");
        return false;
    }

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
        // end with .*
        ptn if ptn.starts_with(".*") || ptn.starts_with(".+") => {
            if input_line.is_empty() {
                return false;
            }

            let ptn_start: usize = if ptn.contains(".*") {
                ptn.find(".*").unwrap()
            } else {
                ptn.find(".+").unwrap()
            };

            let ptn_part = &ptn[ptn_start + 2..];
            eprintln!("=={ptn} ==ptn part is 1 ptn_part: {ptn_part} vs input: {input_line}");

            return input_line.contains(ptn_part);
        }
        ptn if ptn.ends_with(".+$") => {
            if input_line.is_empty() {
                return false;
            }

            let ptn_len = ptn.len();
            let ptn_start = ptn.find(".+").unwrap();

            let ptn_part = &ptn[0..ptn_start];
            eprintln!("=={ptn} ==ptn part is 1 ptn_part: {ptn_part} vs input: {input_line}");

            return match_pattern(input_line, ptn_part);
        }
        ptn if ptn.ends_with(".*") || ptn.ends_with(".*$") => {
            if input_line.is_empty() {
                return false;
            }

            if ptn == ".*" || ptn == ".*$" {
                return true;
            }

            let ptn_len = ptn.len();
            let ptn_start = ptn.find(".*").unwrap();

            let ptn_part = &ptn[0..ptn_start];
            eprintln!("=={ptn} ==ptn part is 1 ptn_part: {ptn_part} vs input: {input_line}");

            if ptn_part.len() == 1 {
                return input_line.starts_with(ptn_part);
            }
            return match_pattern(input_line, ptn_part);
        }

        // check start anchor
        ptn if pattern.starts_with("^") => {
            let to_match = &pattern[1..pattern.len()];
            //println!("matching ^ {}", input_line.starts_with(to_match));
            return input_line.starts_with(to_match);
        }
        // ch,eck ending
        ptn if pattern.ends_with("$") => {
            //println!("{ptn} contains $");
            let to_match = &pattern[0..pattern.len() - 1];
            let regex: Vec<&str> = vec!["\\d", "\\w", "(", ")"];
            if regex.iter().any(|a| to_match.contains(a)) {
                //let mut new_input = &input_line[0..input_line.len() - 1];
                let res = match_pattern(&input_line, to_match);
                //println!("matching {input_line} to {pattern} and res is {res}");
                return res;
            } else {
                return input_line.ends_with(to_match);
            }
        }
        "\\d" => {
            //println!("matching digits {input_line}  ----> {pattern} ---> {}", input_line.chars().any(|e| e.is_ascii_digit()));
            return input_line.chars().any(|e| e.is_ascii_digit());
        }
        "\\d+" => input_line.chars().all(|e| e.is_ascii_digit()),
        "\\d?" => input_line.chars().any(|e| e.is_ascii_digit()) || input_line.len() == 0,
        "d" => input_line.starts_with(|s: char| s.is_ascii_alphabetic()),
        "\\w" => input_line.chars().any(|e| {
            (e.is_ascii_alphanumeric() || e == '_') && !vec!['+', '!', '@', '$'].contains(&e)
        }),
        "\\w+" => {
            for w in input_line.chars() {
                if !match_pattern(&w.to_string(), "\\w") {
                    return false;
                }
            }
            return true;
        }
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

        ptn if ptn.ends_with("*") && !ptn.ends_with("]*") && !ptn.ends_with(")*") => {
            let ptn_len = ptn.len();
            let ptn_before = &ptn[0..ptn_len - 1];

            eprintln!("{input_line} matching ptn end with * {ptn}");

            //edge case pea vs pear*
            // i.e. zero match last char
            if input_line.len() == ptn_before.len() - 1 {
                return ptn_before.starts_with(input_line);
            }
            return match_pattern(input_line, ptn_before);
        }

        // this only handle simplest * match case
        ptn if ptn.contains("*") && !ptn.contains(['[', ']', ')', '(']) && !ptn.contains("\\d") => {
            // simple case apple = apple***
            let ptn_pos = ptn.find("*").unwrap();
            let ptn_before = &ptn[0..ptn_pos];
            let ptn_after = &ptn[ptn_pos + 1..];

            // if input < ptn
            let mut ptn_merged = vec![ptn_before, ptn_after].join("");
            eprintln!("===> Simple * match ptn merged {ptn_merged}");

            // handle cases -> kt matching k*t
            if input_line.len() < pattern.len() {
                if ptn_merged == input_line {
                    return true;
                } else {
                    let mut ptn_before_without = ptn_before;
                    ptn_merged = vec![ptn_before_without, ptn_after].join("");
                    eprintln!(
                        "amtching: {ptn_merged} -> to input: {input_line}, last _char is {:?}, {} ",
                        ptn_before.chars(),
                        ptn_before.ends_with("\\d")
                    );
                    //return input_line == ptn_merged;
                    return match_pattern(input_line, &ptn_merged);
                }
            }

            //ka*t -> match kt and kaat
            if match_pattern(input_line, ptn_before) {
                return input_line.contains(ptn_after);
            } else {
                return false;
            }
        }

        ptn if ptn.contains("}") && ptn.contains("{") => {
            eprintln!("PATTERN CONTAINS{{}}");
            let quant_start = find_index_of_ptn(ptn, "{").unwrap();
            let quant_end = find_index_of_ptn(ptn, "}").unwrap();

            // just return false now if }{ no matching pair is included
            if quant_start > quant_end {
                return false;
            }

            let mut quant_num = &ptn[quant_start + 1..quant_end];

            // normal case considering
            let num = match quant_num.parse::<usize>() {
                Ok(n) => {
                    let letter_before = ptn.chars().nth(quant_start - 1).unwrap().to_string();
                    if vec!["]", ")"].contains(&letter_before.as_str()) {
                        let bracket_start = match &*letter_before {
                            "]" => "[",
                            _ => "(",
                        };
                        let bracket_start_idx = find_index_of_ptn(ptn, bracket_start);
                        eprintln!("bracket start idx: {letter_before}");
                        match bracket_start_idx {
                            Some(start_idx) => {
                                let range_between = &ptn[start_idx + 1..quant_start - 1];
                                let range_before = &ptn[0..start_idx];
                                if match_pattern(input_line, range_before) {
                                    eprintln!(
                                        "===MATCH QUANT WITH [] {range_before} is within {input_line}"
                                    );
                                    return true;
                                } else {
                                    return false;
                                }
                            }

                            None => {}
                        }
                    }
                    eprintln!("Letter to be repeated: {letter_before} for {n} times");
                    let repeated_letter = letter_before.repeat(n);
                    // need to remove the letter to repeat
                    let ptn_new = format!(
                        "{}{}{}",
                        &ptn[0..quant_start - 1],
                        repeated_letter,
                        &ptn[quant_end + 1..]
                    );
                    eprintln!("the new pattern is {ptn_new} -> matching {input_line}");

                    return match_pattern(input_line, &ptn_new);
                }
                Err(e) => {
                    eprintln!("{quant_num} is not digits");
                    return false;
                }
            };

            return false;
        }
        ptn if ptn.contains("[") && ptn.contains("]") => {
            let mut bracket_ptn = "[]".to_string();
            let brac_starter_idx = ptn.find("[").unwrap();
            let mut brac_end_idx = ptn.find("]").unwrap();
            let mut brac_end_inclusive = brac_end_idx + 1;
            let behind_bracket = find_index_of_ptn(ptn, "*");
            match behind_bracket {
                Some(n) => {
                    if n ==brac_end_idx + 1 {
                        brac_end_idx += 1;
                        bracket_ptn = "[]*".to_string();
                        brac_end_inclusive += 1; // ]* is one more than ]
                    }

                },

                None => {}
            }
            
            if brac_end_idx < brac_starter_idx {
                eprintln!("]* appear in front of [");
                return input_line == pattern;
            } else {

                let ptn_before = &ptn[0..brac_starter_idx];
                let ptn_before_len = ptn_before.len();
                let ptn_after = &ptn[brac_end_inclusive..]; //]* len is 2 hardcoded
                let ptn_range = &ptn[brac_starter_idx + 1..brac_end_idx];

                // if input < ptn
                let mut ptn_merged = vec![ptn_before, ptn_after].join("");
                eprintln!("===> {bracket_ptn}  --ptn BEFORE={ptn_before}");
                 eprintln!("===> {bracket_ptn}  --ptn AFTER={ptn_after}");
                eprintln!("===> {bracket_ptn} match --ptn merged={ptn_merged}");

                // just []* pattern
                if ptn_merged.is_empty() {
                    let input_vec: Vec<String> =
                        input_line.chars().map(|a| a.to_string()).collect();
                    return input_vec.iter().all(|a| ptn_range.contains(a));
                }

                // handle cases -> kt matching k*t
                if input_line.len() < pattern.len() {
                    if ptn_merged == input_line {
                        return true;
                    }
                }

                let input_start_match = input_line.find(ptn_before);
                match input_start_match {
                    Some(s_idx) => {
                        let input_range = &input_line[s_idx+ptn_before_len..];
                        let mut input_end_range = "";
                        let mut match_found = false;
                        //match the range of input range to [abc]*
                        for (idx, ch) in input_range.chars().enumerate() {
                            let char_in_ptn = ptn_range.contains(ch);
                            eprintln!(
                                "input = {input_range} at {idx} {ch} in range: [{ptn_range}]* = {char_in_ptn} "
                            );
                            if !char_in_ptn {
                                input_end_range = &input_line[s_idx + idx + 1..];
                                break;
                            }

                            if idx == input_range.len() - 1 {
                                match_found = true;
                                // input completely match pattern
                                eprintln!(
                                    "range len {} = input len {}",
                                    s_idx + input_range.len(),
                                    input_line.len() - 1
                                );
                                if s_idx + input_range.len() == input_line.len() - 1 {
                                    if ptn_after.len() == 0 {
                                        return true; // ptn end at *
                                    } else {
                                        // echo -n "kabacb" | ./your_program.sh -E "k[abc]*b"
                                        return input_range.ends_with(ptn_after);
                                    }
                                } else {
                                    input_end_range = &input_line[s_idx + input_range.len() + 1..];
                                }
                            }
                        }
                        eprintln!(
                            "[]* input end range is {input_end_range}, start with {ptn_after} ? {}",
                            input_end_range.starts_with(ptn_after)
                        );
                        // only handles completely non-pattern match
                        return input_end_range.starts_with(ptn_after);
                    }

                    None => return false,
                }
            }
        }

        ptn if ptn.contains("\\d*") => {
            let ptn_to_detect = "\\d*";
            let ptn_to_detect_len = ptn_to_detect.len();
            let ptn_pos = ptn.find("\\d*").unwrap();
            let ptn_before = &ptn[0..ptn_pos];
            let ptn_after = &ptn[ptn_pos + ptn_to_detect_len..];

            // simple case input does not contain digit
            let ptn_merged = vec![ptn_before, ptn_after].join("");
            eprintln!("merged ptn for \\d* is {ptn_merged}");

            if input_line.to_string() == ptn_merged {
                return true;
            }

            if input_line.starts_with(ptn_before) && input_line.ends_with(ptn_after) {
                let input_start_ends = input_line.find(ptn_before).unwrap() + ptn_before.len();
                let input_end_start = input_line.find(ptn_after).unwrap();
                let input_range = &input_line[input_start_ends..input_end_start];

                return match_pattern(input_range, "\\d");
            }

            return false;
        }

        //check alt | operator
        ptn if pattern.starts_with("(") && pattern.ends_with(")") => {
            //println!("{ptn} eval alternat");
            let ptn_formatted = remove_start_end(ptn);
            let ptn_spilt = ptn_formatted.split("|").collect::<Vec<&str>>();
            if ptn_spilt.len() <= 1 {
                //eprintln!("{:?}", ptn_spilt);
                return false;
            } else {
                let mut contain_alt: Vec<bool> = Vec::new();
                let mut input_compare = input_line.to_string();
                for val in ptn_spilt {
                    eprintln!("(|) eval if {input_compare} contain {val}");
                    // one word straightforward match
                    if input_compare == val {
                        return true;
                    } else if input_compare.contains(val) {
                        input_compare = input_compare.replace(val, "");

                        contain_alt.push(true);
                    } else {
                        contain_alt.push(false);
                    }

                    //eprintln!("the vec is {:?} ", contain_alt);
                }
                return contain_alt.iter().any(|v| *v == true);
            }
        }

        ptn if ptn.starts_with("(") => {
            eprintln!("ptn start with (: {input_line} ----  {ptn}");
            let mut alt_end = ptn.rfind(")").unwrap_or(0);
            let pipe_find = ptn.find("|").unwrap_or(0);
            let mut input_trimmed = false;
            let input_temp = input_line.clone();
            // Handles ?
            if ptn.ends_with("?") {
                let char_before_ptn = ptn
                    .clone()
                    .chars()
                    .nth(ptn.len().saturating_sub(2))
                    .unwrap_or('\0');

                if char_before_ptn != '\0' && input_line.chars().last() == Some(char_before_ptn) {
                    if let Some((last_char_byte_idx, _)) = input_line.char_indices().last() {
                        input_line = &input_line[..last_char_byte_idx];
                        input_trimmed = true;
                    }
                }

                //eprintln!("char before ptn is {char_before_ptn} input is {input_line}");
            }

            // handle )*
            if ptn.ends_with("*") {
                let char_before_ptn = ptn
                    .clone()
                    .chars()
                    .nth(ptn.len().saturating_sub(2))
                    .unwrap_or('\0');

                if char_before_ptn != '\0' && input_line.chars().last() == Some(char_before_ptn) {
                    if let Some((last_char_byte_idx, _)) = input_line.char_indices().last() {
                        input_line = &input_line[..last_char_byte_idx];
                        input_trimmed = true;
                    }
                }

                eprintln!("PTN=()* char before ptn is {char_before_ptn} input is {input_line}");
            }

            if alt_end == 0 || pipe_find == 0 {
                eprintln!("matching  {input_line} -->  {ptn} NO CLOSURE");
                return check_individual_match(input_line, ptn);
            } else if alt_end <= pipe_find {
                eprintln!("NOT CLOSURE!!  {input_line} -->  {ptn} ) appear earlier than |");
                return check_individual_match(input_line, ptn);
            } else {
                let mut ptn_p1_old = ptn.get(1..alt_end).unwrap();
                let ptn_p1_string = format!("({ptn_p1_old})");
                let ptn_p1: &str = &ptn_p1_string;
                eprintln!("( ) | all present => matching {input_line} -------- {ptn_p1}");
                let mut ptn_1_match = match_pattern(input_line, ptn_p1);

                // echo -ne "I see 3 cabbages. Also, I see 4 asparaguss.\nI ate 10 asparagus today" | ./your_program.sh --color=always -E 'I see \d+ (asparagus|cabbage)s?' | cat
                if !ptn_1_match && input_trimmed {
                    ptn_1_match = match_pattern(input_temp, ptn_p1);
                    if ptn_1_match {
                        input_line = input_temp;
                    }
                }

                let ptn_p2 = ptn.get(alt_end + 1..).unwrap();
                if ptn_p2.len() == 0 {
                    return true;
                } else if ptn_1_match {
                    let mut ptn_1_match_start = find_match_inbetween(input_line, ptn_p1_old);
                    //eprintln!("input start is {ptn_1_match_start} and will append {ptn_p2}");

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
                eprintln!("WILDCARD: idx {idx}, c: {c} -> match ptn: {ptn_at_idx}");

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
                        eprintln!(
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
                            //eprintln!("Now reverse match at ptn idx {rev_idx} :  {rev_c} -> {m} ");
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
                            eprintln!(
                                "===QUANT {ptn_quant} MATCHING===starting from {input_line} PREV: {old_input}, search res {c}, res pos {first_letter_to_start}"
                            );
                            let input_line_len = input_line.len();

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
                                eprintln!("enter loop {idx} and val {val}");
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
                                        //eprintln!("ptn before quant: {ptn_before_quant} and quant is {ptn_quant}");
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
                                            eprintln!(
                                                "? reached at input idx {idx} matching {val} to PTN {after_zero_quant} and ptn_quant {ptn_quant}"
                                            );

                                            if val != after_zero_quant {
                                                if val != letter_after_p {
                                                    // println!(
                                                    //     "{val} does not match ptn {}",
                                                    //     after_zero_quant
                                                    // );
                                                    return false;
                                                }
                                                return true;
                                            }
                                            // eprintln!(
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
                                            //eprintln!("{val} vs {pattern}");
                                            return false;
                                        }
                                    }
                                }

                                // reached plus/? sign -> need to have one match
                                if idx == p {
                                    eprintln!(
                                        "reaching idx == p at {idx}, value is {val} and ptn after p {} (idx pattern: {after_p}) LTR after p {letter_after_p}",
                                        &pattern[after_p..]
                                    );
                                    // handle ? after ? should only match world by word
                                    if ptn_quant == "?" {
                                        let ptn_to_match = pattern_char.clone().nth(p + 1).unwrap();
                                        eprintln!(
                                            "AT idx == p NOW, check ? ZERP?ONE quantifier:  val {val} ?== {} ",
                                            ptn_to_match
                                        );

                                        // handle case where not matching and pattern is ending after
                                        if p + 1 == pattern.len() - 1 && val != ptn_to_match {
                                            return false;
                                        }

                                        if val == ptn_to_match && idx == input_line.len() - 1 {
                                            // eprintln!("MATCHED");
                                            return true;
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
                                    eprintln!(
                                        "step {idx}: WHEN {idx} > {p}: checking match after {ptn}: PATTERN {letter_after_p} => VAL {val} INPUT: {input_line}"
                                    );

                                    letter_to_match = pattern.clone().chars().nth(before_p);
                                    if val != letter_to_match.unwrap() {
                                        // if nothing after +
                                        // eprintln!(
                                        //     "NOT MATCHED val is {val} , and to match is {}",
                                        //     letter_to_match.unwrap()
                                        // );
                                        if letter_after_p == '\0' {
                                            return true;
                                        }

                                        let mut pattern_slice = &pattern[p + 1..];

                                        if last_occurance_of_p != p {
                                            pattern_slice = &pattern[last_occurance_of_p + 1..];
                                        }

                                        match (ptn_quant) {
                                            "?" => {
                                                eprintln!(
                                                    "QUANT =? Match {idx} idx::: {ptn} pattern: PTN: {} VAL: {}",
                                                    pattern_slice,
                                                    &input_line[idx..]
                                                );

                                                return input_line[idx..]
                                                    .starts_with(pattern_slice);
                                            }

                                            _ => {
                                                if idx == input_line_len - 1 {
                                                    return val == pattern.chars().last().unwrap();
                                                }

                                                eprintln!(
                                                    "QUANT = +  idx > p matching {idx} idx::: {ptn} pattern: PTN: {} VAL: {}",
                                                    pattern_slice,
                                                    &input_line[idx - 1..]
                                                );

                                                return input_line[idx - 1..]
                                                    .starts_with(pattern_slice);
                                            }
                                        }
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
    //println!("FN CHECK_INDV_MATCHING, matching {input_line} -> pattern {pattern}");
    let pattern_clone: Vec<char> = pattern.chars().collect();
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

    eprintln!("all patterns to check are {:?}", matches_by_index);

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
    let mut new_input = input_line.replace("_", "_ ");
    new_input = new_input.replace(",", ", ");
    new_input = new_input.replace(".", ". ");
    return new_input;
}

pub fn spilt_all_white_space_punc(input_line: &str) -> Vec<String> {
    // let mut new_input = input_line.replace('_', " _ ");
    // new_input = new_input.replace(',', " , ");
    // new_input = new_input.replace('.', " . ");

    //println!("new input is {new_input}");

    let split_input: Vec<String> = input_line
        .split(|t: char| t.is_whitespace())
        .filter(|t| !t.is_empty())
        .map(String::from)
        .collect();

    eprintln!("removed all punc and white space: {:?}", split_input);
    split_input
}

pub fn print_single_matching_line(input_line: &String, ptn: &mut String) -> String {
    let mut pattern = &mut ptn.clone();
    let ptn_clone = pattern.clone().to_string();
    let mut ptn_temp = pattern.to_string();
    if let Some((quant_start, quant_end, new_ptn)) = check_quant_pattern(ptn_clone) {
        ptn_temp = new_ptn;
    }

    pattern = &mut ptn_temp;

    eprintln!("FN___print_single_matching_line__pattern={pattern}");

    // simplest case exact match
    if input_line == pattern {
        return input_line.to_string();
    }

    let mut new_input = remove_underline_and_punc(&input_line);
    if input_line == "_" {
        new_input = "_".to_string();
    }
    let new_ptn = remove_underline_and_punc(&pattern);
    let mut input_slice = new_input.split_whitespace().collect::<Vec<&str>>();
    let mut ptn_slice = new_ptn.split_whitespace().collect::<Vec<&str>>();

    eprintln!(
        "__FN_print_single_matching_line__ INPUT: {:?} **{:?}**",
        input_slice, ptn_slice
    );

    let input_slice_len = input_slice.len();
    let mut res: Vec<char> = Vec::new();
    let mut res_str: Vec<String> = vec![];

    // handle cases with input
    eprintln!(
        "input slice len {input_slice_len} vs ptn_slice len= {}",
        ptn_slice.len()
    );
    if input_slice_len == 1 && ptn_slice.len() == 1 {
        match examine_repeat(input_line, pattern) {
            Some(a) => {
                //eprintln!("the repeat is {a}");
                return a;
            }
            None => {}
        }

        if match_pattern(input_line, pattern) {
            return input_line.to_string();
        } else {
            return "".to_string();
        }
    }

    // handle plural cases
    if pattern.ends_with("s") && !input_line.ends_with("s") {
        //eprintln!("plural case not matching");
        return "".to_string();
    }

    if pattern.contains("\\d+") || pattern.contains("\\w+") {
        eprintln!("PTN  {pattern} contain val {input_line}");
        if match_pattern(input_line, pattern) {
            return input_line.to_string();
        } else {
            return "".to_string();
        }
    }

    // more than 1 word or digit count

    for (idx, val) in input_slice.into_iter().enumerate() {
        eprintln!("{idx}: {val} vs {pattern}");
        if let Some(repeat_matched) = examine_repeat(val, pattern) {
            //println!("repeat: {repeat_matched}");
            return repeat_matched.trim().to_string();
        }

        if match_pattern(val, pattern) {
            eprintln!("==={val} MATCHING {pattern}===");
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
                    //println!("res_st: {:?}  {input_slice_len} vs ptn: {pattern}", res_str);
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

                // already checked and passed
                if pattern.contains("?") {
                    return val.to_string();
                    // let quant_indices: Vec<_> = pattern.match_indices("?").collect();
                    // pattern.replace("?", "");
                    // if pattern == val && pattern.len() > val.len() {
                    //     return val.to_string();
                    // } else {
                    //     for (idx, val ) in val.chars() {

                    //     }
                    // }
                }
                let mut items: Vec<_> = val.match_indices(&*pattern).collect();

                // if spilt_by_hypgens.len() == 0 {
                //     // items = val
                //     // .chars()
                //     // .zip(pattern.chars())
                //     // .filter(|(x, y)| x == y || *y == '?')
                //     // .map(|(x, _)| x)
                //     // .collect();

                // } else {

                // }
                eprintln!("items are {:?} vs {pattern} vs {val}", items);
                if !items.is_empty() {
                    let mut result: String = String::new();
                    let item_len = items.len();
                    for (idx, item) in items.iter() {
                        if *idx != item_len - 1 {
                            result.push_str(format!("{item}\n").as_ref());
                        } else {
                            result.push_str(item)
                        }
                    }

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
