use helper::easy_input;
use std::env::args;

fn main() {
    let input = {
        let run_arg = args().next().unwrap();
        let day_num = run_arg.split('/').last().unwrap();
        easy_input(day_num)
    };

    println!("{}", input);
}
