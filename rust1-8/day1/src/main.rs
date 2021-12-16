fn main() {
    let input = {
        use helper::easy_input;
        use std::env::args;

        let run_arg = args().next().unwrap();
        let day_num = run_arg.split('/').last().unwrap();
        easy_input(day_num)
    };

    day1_part1(&input);
    day1_part2(&input);
}

fn day1_part1(input: &str) {
    let nums = input.lines().filter_map(|x| x.parse::<u32>().ok());
    let mut increases = 0;
    let mut prev: Option<u32> = None;
    for i in nums {
        if prev.is_some() && prev.unwrap() < i {
            increases += 1;
        } else {
            prev = Some(i);
            continue;
        }
        prev = Some(i);
    }
    println!("{}", increases);
}

fn day1_part2(input: &str) {
    let nums = input.lines().filter_map(|x| x.parse::<u32>().ok());
    let mut increases = 0;
    let mut prev: Option<u32> = None;
    for i in nums.collect::<Vec<_>>().windows(3) {
        if prev.is_some() && prev.unwrap() < i.iter().sum() {
            increases += 1;
        } else {
            prev = Some(i.iter().sum());
            continue;
        }
        prev = Some(i.iter().sum());
    }
    println!("{}", increases);
}
