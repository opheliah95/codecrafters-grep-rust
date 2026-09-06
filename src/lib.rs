fn check_all_true(vec: &Vec<bool>) -> bool {
    if vec.len() == 0 {
        return false;
    }
    vec.iter().all(|e| *e == true)
}

fn contain_digits(input: &str) -> bool {
    for c in input.chars() {
        if c.is_ascii_digit() {
            return true;
        }
    }
    return false;
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
