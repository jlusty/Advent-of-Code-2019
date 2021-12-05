fn main() {
    part_2();
}

fn part_1() {
    let min = 158126;
    let max = 624574;

    let mut num_pwd = 0;
    for i in min..max + 1 {
        let i_str = i.to_string();
        let digit_str = &i_str[0..1];
        let mut prev_digit: i32 = digit_str.parse().unwrap();

        let mut equal_digit_cond = false;
        let mut digit_ordering_cond = true;
        let mut pos = 1;
        while pos < 6 && digit_ordering_cond {
            let digit_str = &i_str[pos..pos + 1];
            let digit: i32 = digit_str.parse().unwrap();
            //println!("{}", digit);

            if prev_digit > digit {
                digit_ordering_cond = false;
            } else if prev_digit == digit {
                equal_digit_cond = true;
            }

            prev_digit = digit;
            pos += 1;
        }
        if equal_digit_cond && digit_ordering_cond {
            num_pwd += 1;
        }

        //println!("{}", i);
    }

    println!("{}", num_pwd);
}

fn part_2() {
    let min = 158126;
    let max = 624574;

    let mut num_pwd = 0;
    for i in min..max + 1 {
        let i_str = i.to_string();
        let digit_str = &i_str[0..1];
        let mut prev_digit: i32 = digit_str.parse().unwrap();

        let mut equal_digit_cond = (false, false);
        let mut just_matched = false;
        let mut digit_ordering_cond = true;
        let mut pos = 1;
        while pos < 6 && digit_ordering_cond {
            let digit_str = &i_str[pos..pos + 1];
            let digit: i32 = digit_str.parse().unwrap();
            //println!("{}", digit);

            if prev_digit > digit {
                digit_ordering_cond = false;
            }
            if prev_digit == digit {
                if just_matched {
                    equal_digit_cond.0 = false;
                } else {
                    equal_digit_cond.0 = true;
                }
                just_matched = true;
            } else {
                just_matched = false;
                if equal_digit_cond.0 == true {
                    equal_digit_cond = (false, true);
                }
            }

            prev_digit = digit;
            pos += 1;
        }
        if equal_digit_cond.0 == true {
            equal_digit_cond = (false, true);
        }
        if equal_digit_cond.1 && digit_ordering_cond {
            num_pwd += 1;
        }

        //println!("{}", i);
    }

    println!("{}", num_pwd);
}
