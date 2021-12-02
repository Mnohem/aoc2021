fn main() {
    let input = {
        use helper::easy_input;
        use std::env::args;

        let run_arg = args().next().unwrap();
        let day_num = run_arg.split('/').last().unwrap();
        easy_input(day_num)
    };

    let (depth, x_pos) = day2_part1(&input);
    println!("{}", depth * x_pos);
    let (depth, x_pos) = day2_part2(&input);
    println!("{}", depth * x_pos);
}

fn day2_part1(input: &str) -> (u32, u32) {
    let instructions = input.lines().map(|instr| {
        let mut instr = instr.split_ascii_whitespace();
        (
            instr.next().unwrap(),
            instr.next().unwrap().parse::<u32>().unwrap(),
        )
    });
    let mut depth = 0;
    let mut x_pos = 0;
    for (action, num) in instructions {
        if action == "down" {
            depth += num;
        } else if action == "up" {
            depth -= num;
        } else if action == "forward" {
            x_pos += num;
        }
    }
    (depth, x_pos)
}
fn day2_part2(input: &str) -> (u32, u32) {
    let instructions = input.lines().map(|instr| {
        let mut instr = instr.split_ascii_whitespace();
        (
            instr.next().unwrap(),
            instr.next().unwrap().parse::<u32>().unwrap(),
        )
    });
    let mut depth = 0;
    let mut x_pos = 0;
    let mut aim = 0;
    for (action, num) in instructions {
        if action == "down" {
            aim += num;
        } else if action == "up" {
            aim -= num;
        } else if action == "forward" {
            x_pos += num;
            depth += aim * num;
        }
    }
    (depth, x_pos)
}
