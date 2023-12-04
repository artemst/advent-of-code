use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input_filename = if args.get(1).is_some_and(|arg| arg == "-d") {
        "input-test.txt"
    } else {
        "input.txt"
    };

    let file = File::open(input_filename)?;
    let reader = BufReader::new(file);

    let mut result_for_task_1 = 0;
    let mut result_for_task_2 = 0;
    let mut cards: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let card = Card::from(&line?);
        let wins = card
            .mine
            .iter()
            .filter(|n| card.winning.contains(n))
            .count() as u32;

        if wins > 0 {
            result_for_task_1 += u32::pow(2, wins - 1);
        }

        // add card itself
        if cards.get(card.id - 1).is_some() {
            cards[card.id - 1] += 1;
        } else {
            cards.push(1);
        }
        // add copies
        for i in card.id..card.id + wins as usize {
            if cards.get(i).is_some() {
                cards[i] += cards[card.id - 1];
            } else {
                cards.push(cards[card.id - 1]);
            }
        }
        result_for_task_2 += cards[card.id - 1];
    }

    println!("[1] Result for task 1 = {result_for_task_1}");
    println!("[2] Result for task 2 = {result_for_task_2}");

    Ok(())
}

#[derive(Debug, PartialEq)]
struct Card {
    id: usize,
    winning: Vec<u32>,
    mine: Vec<u32>,
}

impl Card {
    fn from(s: &str) -> Self {
        let (card, card_nums) = s.split_once(':').unwrap();
        let (_, card_id) = card.split_once(' ').unwrap();
        let (winning_str, mine_str) = card_nums.split_once('|').unwrap();
        let winning = winning_str
            .split(' ')
            .filter_map(|num| num.parse().ok())
            .collect();
        let mine = mine_str
            .split(' ')
            .filter_map(|num| num.parse().ok())
            .collect();

        Self {
            id: card_id.trim().parse().ok().unwrap(),
            winning,
            mine,
        }
    }
}
