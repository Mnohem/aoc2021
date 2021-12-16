#[derive(PartialEq, PartialOrd, Eq, Copy, Clone)]
enum BingoTile {
    Marked(u32),
    Unmarked(u32),
}

#[derive(Clone)]
struct BingoCard {
    tiles: Vec<BingoTile>,
}
impl BingoCard {
    fn mark_tile(&mut self, num: u32) {
        use BingoTile::*;

        if self.tiles.contains(&Unmarked(num)) {
            let index = self.tiles.iter().position(|x| x == &Unmarked(num)).unwrap();
            self.tiles[index] = Marked(num);
        }
    }
    fn is_winner(&self) -> bool {
        use BingoTile::*;

        self.tiles
            .chunks(5)
            .any(|x| x.iter().all(|x| matches!(x, Marked(_))))
            || (0..5)
                .map(|i| {
                    self.tiles
                        .iter()
                        .skip(i)
                        .step_by(5)
                        .all(|x| matches!(x, Marked(_)))
                })
                .any(|x| x)
    }
    fn score(&self, last_num: u32) -> u32 {
        use BingoTile::*;
        self.tiles
            .iter()
            .filter_map(|x| match x {
                Marked(_) => None,
                Unmarked(n) => Some(n),
            })
            .sum::<u32>()
            * last_num
    }
}
fn main() {
    let input = {
        use helper::easy_input;
        use std::env::args;

        let run_arg = args().next().unwrap();
        let day_num = run_arg.split('/').last().unwrap();
        easy_input(day_num)
    };

    let mut input = input.split("\n\n");
    let mut num_order = input
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap());
    let mut bingo_cards: Vec<BingoCard> = input
        .map(|x| BingoCard {
            tiles: x
                .trim()
                .split_ascii_whitespace()
                .map(|num| BingoTile::Unmarked(num.parse::<u32>().unwrap()))
                .collect::<Vec<_>>(),
        })
        .collect();
    let mut first_winner_score = 0;
    let mut last_winner_score = 0;
    let mut winners = 0;
    let length = bingo_cards.len();

    for i in num_order.by_ref() {
        for card in bingo_cards.iter_mut() {
            card.mark_tile(i);
            let is_winner = card.is_winner();

            if is_winner && winners == 0 {
                first_winner_score = card.score(i);
                winners += 1;
            } else if is_winner && length - winners == 1 {
                last_winner_score = card.score(i);
            } else if is_winner {
                winners += 1;
            }
        }

        bingo_cards = bingo_cards
            .iter()
            .filter(|x| !x.is_winner())
            .cloned()
            .collect();
    }

    println!(
        "First Winner: {}\nLast Winner: {}",
        first_winner_score, last_winner_score
    );
}
