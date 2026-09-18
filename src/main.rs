use std::env;
use std::io::{self, Read, Write};
use std::process;
mod lib;
use lib::{check_digits, exit_process_errored, remove_start_end, start_end_is_pattern};
mod match_func;
use match_func::{
    count_single_repeat, examine_repeat, match_pattern, print_single_matching_line,
    remove_underline_and_punc, spilt_all_white_space_punc,
};

use crate::match_func::{count_ptn_len, re_formatted_res_with_pattern};

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");
    let mut input_line = String::new();
    io::stdin().read_to_string(&mut input_line).unwrap();
    let mut pattern = env::args().nth(2).unwrap();
    let mut color_always: bool = false;
    if env::args().nth(1).unwrap() == "-o" {
        pattern = env::args().nth(3).unwrap();
        pattern = remove_underline_and_punc(&pattern);
    } else if env::args().nth(1).unwrap() == "--color=always" {
        color_always = true;
        pattern = env::args().nth(3).unwrap();
    }

    let mut split_ptn_by_space = spilt_all_white_space_punc(&pattern);

    let mut split_input_by_space = spilt_all_white_space_punc(&input_line);

    if split_ptn_by_space.len() == 1 {
        if pattern.starts_with("^") && pattern.ends_with("$") {
            let ptn_last = pattern.len();
            let ptn_range = &pattern[1..ptn_last - 1];
            //println!("range is {ptn_range}");
            if *input_line == ptn_range.to_string() {
                if color_always == true {
                    println!("{}", format!("\x1b[01;31m{}\x1b[0m", input_line));
                } else {
                    println!("{input_line}");
                }
                process::exit(0);
            } else {
                exit_process_errored();
            }
        } else if pattern.starts_with("^") {
            let ptn_range = &pattern[1..];
            //println!("here...{}", pattern.clone().chars().nth(1).unwrap());

            if !input_line.starts_with(ptn_range) {
                exit_process_errored();
            }
        } else if pattern.ends_with("$") {
            let ptn_last = pattern.len();
            let ptn_range = &pattern[0..ptn_last - 1];
            if !input_line.ends_with(ptn_range) {
                exit_process_errored();
            }
        }
    }
    //println!("{input_line} line is: {:?}", split_input_by_space);
    if env::args().nth(1).unwrap() == "-o" {
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
                    //eprintln!("DEBUG: original='{}'", r); // See raw value
                    let s = r.trim_end().to_string();
                    // eprintln!("DEBUG: trimmed='{}'", s); // See after trim
                    println!("{}", s.trim());
                }

                process::exit(0);
            } else {
                //eprintln!("res empty");
                process::exit(1);
            }
        } else {
            //echo -ne "mango\n!@#$\nbanana\n+++\ntest123" | ./your_program.sh -E '\w+'
            //println!("{:?} vs PTN {:?}", split_input_by_space, split_ptn_by_space);

            if split_ptn_by_space.len() == split_input_by_space.len() {
                res = handle_single_matching_line(&split_input_by_space, &split_ptn_by_space);
            } else if split_ptn_by_space.len() != split_input_by_space.len() {
                res = handle_single_ptn_to_spaced_txt(&split_input_by_space, &split_ptn_by_space);
            } else if split_ptn_by_space.len() == 0 || split_ptn_by_space.len() == 0 {
                exit_process_errored();
            }
        }

        eprintln!("res: {:?}", res);

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

    if env::args().nth(1).unwrap() != "-E" && env::args().nth(1).unwrap() != "--color=always" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let ptn_len_by_space = split_ptn_by_space.len();
    eprintln!(
        "args contain -E input {:?} ptn  {:?}",
        split_input_by_space, split_ptn_by_space
    );

    let mut res: Vec<String> = Vec::new();

    if split_input_by_space.len() == 1 && split_ptn_by_space.len() == 1 {
        let mut matched = print_single_matching_line(&input_line, &mut pattern);
        if matched.len() == 0 {
            exit_process_errored();
        }
        let matched = matched
            .trim()
            .replace("\n\r", "")
            .replace("\n", "")
            .replace("\r", "")
            .to_string();
        eprintln!("args contain -e input len and ptn len both 1 ->matched [  {matched}  ]");
        match input_line.find(&matched) {
            Some(m) => {
                let m_end = m + matched.len();
                let input_vec = input_line.chars().collect::<Vec<char>>();
                res = input_vec
                    .iter()
                    .enumerate()
                    .map(|(idx, a)| {
                        if color_always {
                            if idx >= m && idx < m_end {
                                format!("\x1b[01;31m{}\x1b[0m", a)
                            } else {
                                a.to_string()
                            }
                        } else {
                            a.to_string()
                        }
                    })
                    .collect::<Vec<String>>();

                if res.len() > 0 {
                    println!("{}", res.join(""));
                    io::stdout().flush().unwrap();
                    process::exit(0)
                } else {
                    eprint!("failed!: {:?}", split_input_by_space);

                    process::exit(1)
                }
            }
            None => {
                let mut matched_char = matched.chars().into_iter().collect::<Vec<char>>();
                let result_str: String = input_line
                    .chars()
                    .map(|c| {
                        if matched_char.contains(&c) {
                            format!("\x1b[01;31m{}\x1b[0m", c)
                        } else {
                            c.to_string()
                        }
                    })
                    .collect();
                if !result_str.is_empty() {
                    println!("{result_str}");
                    process::exit(0)
                } else {
                    eprint!("failed!: {:?}", split_input_by_space);

                    exit_process_errored();
                }
            }
        }
    } else if split_input_by_space.len() == split_ptn_by_space.len() {
        res = handle_single_matching_line(&split_input_by_space, &split_ptn_by_space);
    } else {
        res = handle_single_ptn_to_spaced_txt(&split_input_by_space, &split_ptn_by_space);
    }

    let mut res_trim: Vec<&str> = res
        .iter()
        .flat_map(|t| t.lines())
        .map(|t| t.trim())
        .filter(|t| !t.is_empty())
        .collect();

    let mut res_temp = re_formatted_res_with_pattern(res_trim.clone(), &pattern);
    //eprintln!("res temp {:?}", res_temp);

    let ptn_len_counted = count_ptn_len(split_ptn_by_space.clone());
    if ptn_len_counted > 1 {
        let res_temp_len = res_temp.len();
        let quotient = res_temp_len % ptn_len_counted;

        if quotient > 0 {
            // need to improve
            res_temp = res_temp[0..res_temp_len - quotient].to_vec();

            // eprintln!(
            //     "quotent={quotient}, len is {res_temp_len} and ptn_len_counted={ptn_len_counted}, res_temp={:?}",
            //     res_temp
            // );
        }
    }
    res_trim = res_temp
        .iter()
        .filter(|s| !s.is_empty())
        .map(String::as_str)
        .collect();
    //eprintln!("res trimed is {:?}  and res is {:?} ", res_trim, res);
    if color_always {
        let input_spilt_by_space_only: Vec<String> = input_line
            .split_inclusive('\n')
            .map(|c| c.to_string())
            .collect();
        eprintln!(
            "the input spilt by space is {:?} vs res {:?}",
            input_spilt_by_space_only, res_trim
        );
        res = input_spilt_by_space_only
            .iter()
            .map(|mut a| {
                // Strip trailing punctuation like ',' or '.' for comparison
                let clean_a = a.trim_end_matches(|c| c == ',' || c == '.');

                // Check if `clean_a` directly exists in `res_trim` or matches any pattern entry
                let is_matched =
                    res_trim.contains(&clean_a) || res_trim.iter().any(|&r| clean_a == r);

                if is_matched {
                    // Retain any trailing punctuation outside the color codes
                    format_red_output(a)
                } else {
                    // Unmatched tokens remain plain/uncolored
                    // let words: Vec<String> =
                    //     a.split_inclusive('\n').map(|c| c.to_string()).collect();
                    // println!("the words are: {:?}", words);
                    format_and_filter_indv_letter_to_red(res_trim.clone(), &a, ptn_len_counted)
                }
            })
            .filter(|m| !m.is_empty())
            .collect::<Vec<String>>();
    } else {
        res = res
            .iter()
            .map(|a| a.trim().to_string())
            .filter(|a| !a.is_empty())
            .collect::<Vec<String>>();
    }

    eprintln!(
        "handle single ptn to sentence: {:?}  vd ptn_len {ptn_len_by_space}",
        res
    );

    let input_has_space = input_line.split(" ").collect::<Vec<&str>>().len();
    eprintln!("old input has space {input_has_space}");

    if res.len() > 0 && input_has_space == 1 {
        if !color_always {
            let res_udpated = res
                .iter()
                .map(|a| {
                    let cleaned = a.replace("\n", "");

                    cleaned.trim().to_string()
                })
                .collect::<Vec<String>>();

            eprintln!(" input_has_space == 1 -=> res updated {:?}", res_udpated);
            let spilt_input_whitespace_only: Vec<String> = input_line.split_whitespace().map(|a| a.to_string()).collect();
            for r in res_udpated {
                spilt_input_whitespace_only
                    .iter()
                    .filter(|a| a.contains(&r))
                    .for_each(|a| println!("{}", a));
            }
        } else {
            println!("{}", res.join(""));
        }
    } else if res.len() > 0
        && env::args().nth(1).unwrap() == "-E"
        && !color_always
        && !input_line.contains("\n")
    {
        if res.len() >= split_ptn_by_space.len() {
            println!("{input_line}");
            process::exit(0)
        }

        //eprintln!("input spilt len is {:?}", split_ptn_by_space);
        process::exit(1)
    } else if res.len() > 0 {
        if ptn_len_by_space == 1 || color_always {
            print_with_newline(&res);
        } else if !color_always {
            for r in res.chunks(ptn_len_by_space) {
                if r.len() == ptn_len_by_space {
                    println!("{}", r.join(" "));
                }
            }
        }
        process::exit(0)
    } else {
        eprintln!(
            "failed!: {:?} LEN={} vs PTN spilt {:?}",
            split_input_by_space,
            res.len(),
            ptn_len_by_space
        );

        process::exit(1)
    }
}

fn format_and_filter_indv_letter_to_red(
    res_trim: Vec<&str>,
    input: &str,
    ptn_len: usize,
) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut highlight_mask = vec![false; chars.len()];
    let mut match_found = false;

    let mut search_offset = 0;
    let res_count = res_trim.len();

    for w in res_trim.into_iter() {
        if w.is_empty() || search_offset >= input.len() {
            continue;
        }
        eprintln!(
            "the input is {input} vs {w} and highlight_mask: {:?}",
            highlight_mask
        );

        // Find match strictly after the previous search offset
        if let Some(byte_idx) = input[search_offset..].find(w) {
            let abs_byte_idx = search_offset + byte_idx;

            // Map byte indices back to character indices
            let char_start = input[..abs_byte_idx].chars().count();
            let w_char_len = w.chars().count();

            for i in char_start..char_start + w_char_len {
                if i < highlight_mask.len() {
                    highlight_mask[i] = true;
                }
            }

            match_found = true;
            // Advance search_offset past the matched segment
            search_offset = abs_byte_idx + w.len();
        }
    }

    let true_count = highlight_mask.iter().filter(|a| **a).count();
    if !match_found || true_count < ptn_len {
        return String::new();
    }

    // Reconstruct string with red formatting where masked
    let mut result = String::new();
    for (i, &c) in chars.iter().enumerate() {
        if highlight_mask[i] {
            result.push_str(&format_red_output(&c.to_string()));
        } else {
            result.push(c);
        }
    }

    result
}

fn format_red_output(a: &String) -> String {
    if a.ends_with(',') || a.ends_with('.') {
        let a_len = a.len();
        format!("\x1b[01;31m{}\x1b[0m{}", &a[0..a_len - 1], &a[a_len - 1..])
    } else {
        format!("\x1b[01;31m{}\x1b[0m", a)
    }
}

fn print_with_newline(res: &Vec<String>) {
    for (idx, r) in res.iter().enumerate() {
        if r.ends_with("\n") {
            print!("{r}");
        } else {
            if idx < res.len() - 1 {
                print!("{r} ");
            } else {
                println!("{r}");
            }
        }
    }
    //println!("");
    io::stdout().flush();
}

fn handle_sentence_ptn(sentences: &Vec<String>, pattern: String) -> Vec<String> {
    let mut sentence_collection: Vec<String> = Vec::new();

    for sentence in sentences.iter() {
        let sentence_spilt: Vec<String> =
            sentence.split_whitespace().map(|a| a.to_string()).collect();
        let ptn_split: Vec<String> = pattern.split_whitespace().map(|a| a.to_string()).collect();

        let res = handle_single_ptn_to_spaced_txt(&sentence_spilt, &ptn_split);

        if !res.is_empty() && !(res.len() == 1 && matches!(res[0].as_str(), "\n" | "\r" | "\n\r")) {
            // Group tokens into separate lines whenever a "\n" token appears
            let lines = res.split(|token| token == "\n" || token == "\r\n");

            // Format each line cleanly
            for line_tokens in lines {
                let cleaned_line = line_tokens
                    .iter()
                    .map(|token| token.trim()) // Strip spaces and hidden newlines off each token
                    .filter(|token| !token.is_empty())
                    .collect::<Vec<&str>>()
                    .join(" "); // Join with EXACTLY one space (prevents trailing spaces)

                if !cleaned_line.is_empty() {
                    sentence_collection.push(cleaned_line);
                }
            }
        }
    }

    sentence_collection
}

fn handle_single_ptn_to_spaced_txt(
    split_input_by_space: &Vec<String>,
    split_ptn_by_space: &Vec<String>,
) -> Vec<String> {
    let mut res = Vec::new();
    let mut start: usize = 0;
    let input_len = split_input_by_space.len();
    let ptn_len = split_ptn_by_space.len();
    let mut temp_input_len = 0;
    eprintln!(
        "====FN handle_single_ptn_to_spaced_txt=== {:?} vs {:?}",
        split_input_by_space, split_input_by_space
    );
    while start < split_input_by_space.len() {
        for (ptn_idx, ptn) in split_ptn_by_space.iter().enumerate() {
            //start = 0;
            let mut input_to_start_at = &split_input_by_space[start..];
            if ptn_idx != ptn_len - 1 && input_to_start_at.is_empty() {
                eprintln!(
                    "--AT {ptn_idx} matching {ptn} --- to input (  {:?}  ) start ={start}",
                    input_to_start_at
                );
                start -= temp_input_len;
                input_to_start_at = &split_input_by_space[start..];
            }
            for (idx, i) in input_to_start_at.iter().enumerate() {
                //println!("index: {start} matching: {i} == {ptn}");
                temp_input_len = input_to_start_at.len();
                let mut i_str = i.replace(".", "").replace(",", ""); // handle plural cases

                let mut res_str = String::new();
                res_str = print_single_matching_line(&i_str, &mut ptn.to_string());

                eprintln!(
                    "resuot: ---{res_str}---idx: {idx}--len is {}",
                    res_str.len()
                );
                start += 1;
                if !res_str.trim().is_empty() || res_str.len() > 0 {
                    if idx == split_input_by_space.len() - 1 {
                        res.push(res_str);
                        //println!("===break===");

                        //println!("idx reached ==== {start}=== idx {idx}");
                    } else {
                        res.push(format!("{res_str}\n"));
                        break;
                    }
                } else if res_str.trim().is_empty() {
                    if idx == input_to_start_at.len() - 1 {
                        eprintln!(
                            "NO MATCH! for {:?} at {ptn_idx} {ptn} NOT matching {i} whose idx is {idx}, LEN ={}",
                            input_to_start_at,
                            input_to_start_at.len()
                        );

                        eprintln!("END OF INDEX");
                        // start = 0;
                    }
                }
            }
            eprintln!("--at {ptn_idx} matching... {ptn} finished-- start ={start}");
        }

        if start != split_input_by_space.len() && split_ptn_by_space.len() > 1 {
            res.push("\n".to_string())
        }
    }

    return res;
}

fn handle_single_matching_line(
    split_input_by_space: &Vec<String>,
    split_ptn_by_space: &Vec<String>,
) -> Vec<String> {
    let mut res = Vec::new();
    eprintln!("===FN handle_single_matching_line===INPUT AND PTN LEN MATCH");
    for (i, p) in split_input_by_space.iter().zip(split_ptn_by_space) {
        let mut p_str = p.to_string();

        let res_str = print_single_matching_line(&i.to_string(), &mut p_str);
        //eprintln!(" matching {i} -> {p_str} res-str: {res_str} -o arg");
        if res_str.len() > 0 {
            res.push(res_str)
        }
    }
    eprintln!(
        "the res string length : {} and res is {:?} ",
        res.len(),
        res
    );

    if !env::args().any(|x| x == "-o") {
        let test = res.join("");
        let input = split_input_by_space.join("");
        if !input.contains(&test) {
            return Vec::new();
        }
        let cleaned = res
            .iter()
            .map(|x| x.trim().to_string())
            .map(|x| x.replace("\n", ""))
            .collect();

        eprintln!("cleaned is {:?}", cleaned);
        return cleaned;
    }

    if res.len() >= split_ptn_by_space.len() {
        return res;
    } else {
        return Vec::new();
    }
}
