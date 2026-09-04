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