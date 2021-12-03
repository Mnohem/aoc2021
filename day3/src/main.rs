fn main() {
    let input = {
        use helper::easy_input;
        use std::env::args;

        let run_arg = args().next().unwrap();
        easy_input(run_arg.split('/').last().unwrap())
    };

    //Solution starts here
    let fold = input.lines().fold([0; 12], |mut acc, st| {
        for (i, c) in st.bytes().enumerate() {
            if c == b'1' {
                acc[i] += 1i32;
            } else if c == b'0' {
                acc[i] -= 1i32;
            }
        }
        acc
    });

    let gamma: u32 = fold
        .iter()
        .enumerate()
        .map(|(i, &x)| (x.is_positive() as u32) << i)
        .sum();
    let epsilon = !gamma & 0xFFF;

    println!("{:012b}", gamma);
    println!("{:012b}", epsilon);
    println!("{:?}", gamma * epsilon);
}
