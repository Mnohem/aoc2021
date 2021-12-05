use std::collections::HashMap;

fn main() {
    let input = {
        use helper::easy_input;
        use std::env::args;

        let run_arg = args().next().unwrap();
        let day_num = run_arg.split('/').last().unwrap();
        easy_input(day_num)
    };

    let nums = input
        .lines()
        .map(|x| {
            x.split(" -> ")
                .map(|x| x.split(',').map(|x| x.parse::<i32>().unwrap()))
        })
        .flatten()
        .flatten();

    let mut graph = HashMap::new();

    for line in nums.collect::<Vec<i32>>().chunks(4) {
        for point in points_under_line((line[0], line[1]), (line[2], line[3])) {
            *graph.entry(point).or_insert(0) += 1;
        }
    }

    println!("{}", graph.values().filter(|&&x| x > 1).count());
}

fn points_under_line((mut x1, mut y1): (i32, i32), (x2, y2): (i32, i32)) -> Vec<(i32, i32)> {
    let mut points = vec![];

    if x1 == x2 {
        if y1 > y2 {
            while y1 >= y2 {
                points.push((x1, y1));
                y1 -= 1;
            }
        } else if y1 < y2 {
            while y1 <= y2 {
                points.push((x1, y1));
                y1 += 1;
            }
        } else if y1 == y2 {
            return vec![(x1, y1)];
        }
    } else if y1 == y2 {
        if x1 > x2 {
            while x1 >= x2 {
                points.push((x1, y1));
                x1 -= 1;
            }
        } else if x1 < x2 {
            while x1 <= x2 {
                points.push((x1, y1));
                x1 += 1;
            }
        }
    } else {
        let direction = [-2 * (!(x1 < x2) as i32) + 1, -2 * (!(y1 < y2) as i32) + 1];
        while x1 != x2 && y1 != y2 {
            points.push((x1, y1));
            x1 += direction[0];
            y1 += direction[1];
        }
        points.push((x2, y2));
    }
    points
}
