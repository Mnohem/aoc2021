fn main() {
    let input = {
        use helper::easy_input;
        use std::env::args;

        let run_arg = args().next().unwrap();
        let day_num = run_arg.split('/').last().unwrap();
        easy_input(day_num)
    };

    let age_groups: Vec<u64> = (0..=8)
        .map(|age| {
            input
                .split(',')
                .map(|x| x.parse::<u64>().unwrap())
                .filter(|&x| x == age)
                .count() as u64
        })
        .collect();

    println!("{}", simulate(&age_groups, 80));
    println!("{}", simulate(&age_groups, 256));
}
fn simulate(age_groups: &[u64], generations: usize) -> u64 {
    //I kinda copied this solution from the subreddit
    //Never solved a problem like this, so could be useful elsewhere
    (0..generations)
        .fold(age_groups.to_vec(), |mut v, _| {
            let new_fish = v[0];
            v.rotate_left(1);
            //[0] goes to [8], which is good, but also need [0] at [6]
            //since a fish age 0 becomes a fish age 6 and makes a fish age 8
            v[6] += new_fish;
            v
        })
        .iter()
        .sum::<u64>()
}
