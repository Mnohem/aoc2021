use std::collections::HashMap;

fn main() {
    let input = {
        use helper::easy_input;
        use std::env::args;

        let run_arg = args().next().unwrap();
        let day_num = run_arg.split('/').last().unwrap();
        easy_input(day_num)
    };

    let entries = input.lines().map(|x| x.split(" | "));

    let unique_segments = entries
        .clone()
        .flat_map(|x| x.skip(1).next().unwrap().split(' '))
        .filter(|x| match x.len() {
            2 | 3 | 4 | 7 => true,
            _ => false,
        })
        .count();
    println!("{}", unique_segments);
    let total: u64 = entries
        .map(|mut x| decode_entry(x.next().unwrap(), x.next().unwrap()))
        .sum();
    println!("{}", total);
}

fn decode_entry(code_str: &str, num_str: &str) -> u64 {
    let digit_codes = code_str.split(' ');

    let one_code = &digit_codes.clone().find(|x| x.len() == 2).unwrap();
    let seven_code = &digit_codes.clone().find(|x| x.len() == 3).unwrap();
    let four_code = &digit_codes.clone().find(|x| x.len() == 4).unwrap();
    let eight_code = &digit_codes.clone().find(|x| x.len() == 7).unwrap();

    let digit_codes = digit_codes
        .filter(|x| x != one_code && x != seven_code && x != four_code && x != eight_code);

    let mut decode_map = HashMap::new();

    let top_seg_code = seven_code
        .chars()
        .find(|x| !one_code.contains(&x.to_string()))
        .unwrap();

    let mut partial_nine = four_code.to_string();
    partial_nine.push(top_seg_code);

    let nine_code = digit_codes
        .clone()
        .find(|x| contains_chars(x, &partial_nine))
        .unwrap();
    let digit_codes = digit_codes.filter(|x| x != &nine_code);

    let ll_seg_code = one_diff_char(&eight_code, nine_code).unwrap();

    let mut three_and_five = digit_codes
        .clone()
        .filter(|x| !x.contains(&ll_seg_code.to_string()));
    let mut zero_and_six = digit_codes.clone().filter(|x| x.len() == 6);

    let mut five_code = "";
    let mut six_code = "";
    for code1 in three_and_five.clone() {
        for code2 in zero_and_six.clone() {
            let mut maybe_six = (ll_seg_code.to_string() + code1.clone())
                .chars()
                .collect::<Vec<_>>();
            maybe_six.sort();
            let mut maybe_also_six = code2.chars().collect::<Vec<_>>();
            maybe_also_six.sort();
            if maybe_six == maybe_also_six {
                five_code = code1.clone();
                six_code = code2.clone();
            }
        }
    }
    let zero_code = zero_and_six.find(|&x| x != six_code).unwrap();
    let three_code = three_and_five.find(|&x| x != five_code).unwrap();
    let two_code = digit_codes
        .clone()
        .find(|&x| x != five_code && x != six_code && x != zero_code && x != three_code)
        .unwrap();

    decode_map.insert(sort_str(zero_code), 0);
    decode_map.insert(sort_str(one_code), 1);
    decode_map.insert(sort_str(two_code), 2);
    decode_map.insert(sort_str(three_code), 3);
    decode_map.insert(sort_str(four_code), 4);
    decode_map.insert(sort_str(five_code), 5);
    decode_map.insert(sort_str(six_code), 6);
    decode_map.insert(sort_str(seven_code), 7);
    decode_map.insert(sort_str(eight_code), 8);
    decode_map.insert(sort_str(nine_code), 9);

    num_str
        .split(' ')
        .enumerate()
        .map(|(i, x)| {
            let k = sort_str(x);
            decode_map[&k] * 10u32.pow(3 - i as u32)
        })
        .sum::<u32>() as u64
}

fn contains_chars(this: &str, chars: &str) -> bool {
    for c in chars.chars() {
        if this.contains(&c.to_string()) {
            continue;
        }
        return false;
    }
    true
}

fn one_diff_char(this: &str, other: &str) -> Option<char> {
    for c in this.chars() {
        if !other.contains(&c.to_string()) {
            return Some(c);
        }
    }
    None
}

fn sort_str(this: &str) -> String {
    let mut vec = this.chars().collect::<Vec<_>>();
    vec.sort();
    vec.into_iter().collect::<String>()
}
