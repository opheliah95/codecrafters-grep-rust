use std::env;
use std::io::{self, Read, Write};
use std::process;
mod lib;
use lib::{check_digits, exit_process_errored, remove_start_end, start_end_is_pattern};
mod match_func;
use match_func::{
    match_pattern, print_single_matching_line, remove_underline_and_punc,
    spilt_all_white_space_punc,
};

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
            let ptn_range = &pattern[1..ptn_last];
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
                eprint!("failed!: {:?}", split_input_by_space);

                exit_process_errored();
            }
        }
    } else if split_input_by_space.len() == split_ptn_by_space.len() {
        res = handle_single_matching_line(&split_input_by_space, &split_ptn_by_space);
    } else {
        res = handle_single_ptn_to_spaced_txt(&split_input_by_space, &split_ptn_by_space);
    }

    let res_trim = res
        .iter()
        .map(|t| t.trim())
        .filter(|t| !t.is_empty())
        .collect::<Vec<&str>>();

    if color_always {
        res = split_input_by_space
            .iter()
            .map(|a| {
                if res_trim.contains(&a.as_str()) {
                    format!("\x1b[01;31m{}\x1b[0m", a)
                } else {
                    a.trim().to_string()
                }
            })
            .collect::<Vec<String>>();
    } else {
        res = res
            .iter()
            .map(|a| a.trim().to_string())
            .filter(|a| !a.is_empty())
            .collect::<Vec<String>>();
    }

    eprintln!(
        "handle single ptn to sentence: {:?}  and {ptn_len_by_space}",
        res
    );

    let input_has_space = input_line.split_whitespace().collect::<Vec<&str>>().len();
    eprintln!("old input has space {input_has_space}");

    if res.len() > 0 && input_has_space == 1 {
        if !color_always {
            println!("{}", input_line);
        } else {
            println!("{}", res.join(""));
        }
    } else if res.len() > 0 && res.len() >= ptn_len_by_space {
        for r in res.chunks(ptn_len_by_space) {
            if r.len() == ptn_len_by_space {
                println!("{}", r.join(" "));
            }
        }
        process::exit(0)
    } else {
        eprint!("failed!: {:?}", split_input_by_space);

        process::exit(1)
    }
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
    let mut new_ptn_spilt = split_ptn_by_space.to_vec();

    if ptn_len == 1 {
        let ptn = split_ptn_by_space.iter().nth(0).unwrap();
        for i in (0..input_len - 1) {
            new_ptn_spilt.push(ptn.to_string());
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
                let mut res_str = String::new();

                if env::args().any(|arg| arg == "-o") {
                    res_str = print_single_matching_line(&i_str, &mut ptn.to_string());
                } else {
                    if match_pattern(i, ptn) {
                        res_str = i.to_string();
                    }
                }
                //println!("resuot: ---{res_str}---{idx}");
                start += 1;
                if res_str.to_string().trim().len() > 0 {
                    if idx == split_input_by_space.len() - 1 {
                        res.push(res_str);
                        //println!("===break===");

                        //println!("idx reached ==== {start}=== idx {idx}");
                    } else {
                        res.push(format!("{res_str}\n"));
                        break;
                    }
                }
            }
        }

        if start != split_input_by_space.len() {
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
