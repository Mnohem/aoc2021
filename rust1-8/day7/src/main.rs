fn main() {
    let input = {
        use helper::easy_input;
        use std::env::args;

        let run_arg = args().next().unwrap();
        let day_num = run_arg.split('/').last().unwrap();
        easy_input(day_num)
    };

    let mut positions: Vec<i64> = input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    positions.sort();

    let length = positions.len();

    let median = positions[length / 2];

    let mean: f64 = positions.iter().sum::<i64>() as f64 / length as f64;
    let means = [mean.ceil() as i64, mean.floor() as i64].into_iter();

    let total_fuel_part1 = positions
        .iter()
        .map(|x| (x - median).abs() as u64)
        .sum::<u64>();

    let total_fuel_part2 = means
        .map(|mean| {
            positions
                .iter()
                .map(|x| {
                    let n = (x - mean).abs() as u64;
                    (n.pow(2) + n) / 2
                })
                .sum::<u64>()
        })
        .min()
        .unwrap();

    println!("{}\n{}", total_fuel_part1, total_fuel_part2);
}
