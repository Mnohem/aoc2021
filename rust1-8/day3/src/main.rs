fn main() {
    let input = {
        use helper::easy_input;
        use std::env::args;

        let run_arg = args().next().unwrap();
        easy_input(run_arg.split('/').last().unwrap())
    };

    day3_part1(&input);
    day3_part2(&input);
}

fn day3_part2(input: &str) {
    let mut valid_nums_o2: Vec<_> = input.lines().collect();
    let mut bits = common_bits(input.lines());
    let mut i = 0;
    let o2 = loop {
        valid_nums_o2.retain(|x| {
            let bit = x.bytes().nth(i).unwrap() - b'0';
            !((bits[i] >= 0) ^ (bit == 1))
        });
        if valid_nums_o2.len() == 1 {
            break u32::from_str_radix(&valid_nums_o2[0], 2).unwrap();
        } else if valid_nums_o2.is_empty() {
            panic!("No valid nums");
        }
        bits = common_bits(valid_nums_o2.iter());
        i += 1;
    };

    let mut valid_nums_co2: Vec<_> = input.lines().collect();
    bits = common_bits(input.lines());
    i = 0;
    let co2 = loop {
        valid_nums_co2.retain(|x| {
            let bit = x.bytes().nth(i).unwrap() - b'0';
            (bits[i] >= 0) ^ (bit == 1)
        });
        if valid_nums_co2.len() == 1 {
            break u32::from_str_radix(&valid_nums_co2[0], 2).unwrap();
        } else if valid_nums_co2.is_empty() {
            panic!("No valid nums");
        }
        bits = common_bits(valid_nums_co2.iter());
        i += 1;
    };

    println!("{}", co2 * o2);
}

fn common_bits<SI, T>(lines: SI) -> [i32; 12]
where
    SI: Iterator<Item = T>,
    T: std::string::ToString,
{
    let bits = lines.fold([0; 12], |mut acc, st| {
        for (i, c) in st.to_string().bytes().enumerate() {
            if c == b'1' {
                acc[i] += 1i32;
            } else if c == b'0' {
                acc[i] -= 1i32;
            }
        }
        acc
    });
    bits
}

fn day3_part1(input: &str) {
    let mut bits = common_bits(input.lines());
    bits.reverse();
    let gamma: u32 = bits
        .iter()
        .enumerate()
        .map(|(i, &x)| (x.is_positive() as u32) << i)
        .sum();
    let epsilon = !gamma & 0xFFF;
    println!("{:?}", gamma * epsilon);
}
