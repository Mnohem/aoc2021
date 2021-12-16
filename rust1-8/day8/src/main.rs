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
        .flat_map(|x| x.skip(1).next().unwrap().split(' '))
        .filter(|x| match x.len() {
            2 | 3 | 4 | 7 => true,
            _ => false,
        })
        .count();
    println!("{}", unique_segments);
}

const TOP: u8 = 0b01000000;
const UPPER_LEFT: u8 = 0b00100000;
const UPPER_RIGHT: u8 = 0b00010000;
const MIDDLE: u8 = 0b00001000;
const LOWER_LEFT: u8 = 0b00000100;
const LOWER_RIGHT: u8 = 0b00000010;
const BOTTOM: u8 = 0b00000001;

fn decode_entry(code_str: &str, num_str: &str) -> Option<u64> {
    let digit_codes = code_str.split(' ');

    let one_code = &digit_codes.find(|x| x.len() == 2)?;
    let seven_code = &digit_codes.find(|x| x.len() == 3)?;
    let four_code = &digit_codes.find(|x| x.len() == 4)?;
    let eight_code = &digit_codes.find(|x| x.len() == 7)?;

    let digit_codes = digit_codes
        .filter(|x| x != one_code && x != seven_code && x != four_code && x != eight_code);

    let mut decode_map = HashMap::new();

    let top_seg_code = seven_code
        .chars()
        .find(|x| !one_code.contains(&x.to_string()))?;

    let mut partial_nine = four_code.to_string();
    partial_nine.push(top_seg_code);

    let nine_code = digit_codes.find(|x| contains_chars(x, &partial_nine))?;
    let digit_codes = digit_codes.filter(|x| x != &nine_code);

    let bottom_seg_code = one_diff_char(nine_code, &partial_nine)?;
    let ll_seg_code = one_diff_char(&eight_code, nine_code)?;

    let three_and_five = digit_codes.filter(|x| !x.contains(&ll_seg_code.to_string()));
    let zero_and_six = digit_codes.filter(|x| x.len() == 6);

    let mut five_code = "";
    let mut six_code = "";
    for code1 in three_and_five {
        for code2 in zero_and_six {
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
    let zero_code = zero_and_six.find(|x| x != six_code)?;
    let three_code = three_and_five.find(|x| x != five_code)?;
    let two_code = digit

    decode_map.insert(zero_code, 0);
    decode_map.insert(*one_code, 1);
    decode_map.insert(two_code, 2);
    decode_map.insert(three_code, 3);
    decode_map.insert(four_code, 4);
    decode_map.insert(five_code, 5);
    decode_map.insert(six_code, 6);
    decode_map.insert(seven_code, 7);
    decode_map.insert(eight_code, 8);
    decode_map.insert(nine_code, 9);

    let encoded_num = num_str.split(' ');
}

fn bin_segment_to_num(bin_segment: u8) -> u8 {
    match bin_segment {
        TOP | UPPER_LEFT | UPPER_RIGHT | MIDDLE | LOWER_RIGHT | BOTTOM => 9,
        TOP | UPPER_LEFT | UPPER_RIGHT | MIDDLE | LOWER_LEFT | LOWER_RIGHT | BOTTOM => 8,
        TOP | UPPER_RIGHT | LOWER_RIGHT => 7,
        TOP | UPPER_LEFT | MIDDLE | LOWER_LEFT | LOWER_RIGHT | BOTTOM => 6,
        TOP | UPPER_LEFT | MIDDLE | LOWER_RIGHT | BOTTOM => 5,
        UPPER_LEFT | UPPER_RIGHT | MIDDLE | LOWER_RIGHT => 4,
        TOP | UPPER_RIGHT | MIDDLE | LOWER_RIGHT | BOTTOM => 3,
        TOP | UPPER_RIGHT | MIDDLE | LOWER_LEFT | BOTTOM => 2,
        UPPER_RIGHT | LOWER_RIGHT => 1,
        TOP | UPPER_LEFT | UPPER_RIGHT | LOWER_LEFT | LOWER_RIGHT | BOTTOM => 0,
    }
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
