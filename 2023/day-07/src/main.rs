use core::panic;
use std::{collections::HashMap, io};

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input_filename = if args.get(1).is_some_and(|arg| arg == "-d") {
        "input-test.txt"
    } else {
        "input.txt"
    };

    let file = std::fs::read_to_string(input_filename)?;

    let result_for_task_1 = get_value_for_task(&file, false);
    let result_for_task_2 = get_value_for_task(&file, true);

    println!("[1] Result for task 1 = {result_for_task_1}");
    println!("[2] Result for task 2 = {result_for_task_2}");

    Ok(())
}

fn get_value_for_task(str: &str, joker: bool) -> u64 {
    let mut data: Vec<(HandType, u64)> = str
        .trim()
        .split('\n')
        .map(|row| row.split_once(' ').unwrap())
        .map(|(hand, bid)| {
            (
                hand.chars()
                    .map(|ch| convert_card_to_comparable(ch, joker))
                    .collect::<String>(),
                bid.trim().parse().unwrap(),
            )
        })
        .map(|(hand, bid)| (HandType::from(&hand, &hand, joker), bid))
        .collect();

    data.sort_by(|(a, _), (b, _)| a.cmp(b));

    data.iter()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i + 1) as u64)
        .sum::<u64>()
}

fn convert_card_to_comparable(card: char, joker: bool) -> char {
    if card.is_ascii_digit() {
        return card;
    }
    if joker && card == 'J' {
        return '1';
    }
    match card {
        'T' => 'A',
        'J' => 'B',
        'Q' => 'C',
        'K' => 'D',
        'A' => 'E',
        _ => panic!("Unknown card: {card}"),
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    One(String),       // 5 different cards
    Pair(String),      // 2 + 1 + 1 + 1
    TwoPairs(String),  // 2 + 2 + 1
    Three(String),     // 3 + 1 + 1
    FullHouse(String), // 3 + 2
    Four(String),      // 4 + 1
    Five(String),      // 5
}

impl HandType {
    fn from(cards: &str, hand_val: &str, joker: bool) -> Self {
        let val_to_cnt = cards.chars().fold(HashMap::new(), |mut acc, val| {
            acc.entry(val).and_modify(|cnt| *cnt += 1).or_insert(1);
            acc
        });

        if joker && cards.contains('1') {
            val_to_cnt
                .keys()
                .map(|&k| {
                    let cards_variation = cards.replace('1', &k.to_string());
                    HandType::from(&cards_variation, hand_val, false)
                })
                .max()
                .unwrap()
        } else {
            let hand_val = hand_val.to_string();
            match val_to_cnt.len() {
                1 => HandType::Five(hand_val),
                2 => {
                    if val_to_cnt.values().max() == Some(&4) {
                        HandType::Four(hand_val)
                    } else {
                        HandType::FullHouse(hand_val)
                    }
                }
                3 => {
                    if val_to_cnt.values().max() == Some(&3) {
                        HandType::Three(hand_val)
                    } else {
                        HandType::TwoPairs(hand_val)
                    }
                }
                4 => HandType::Pair(hand_val),
                5 => HandType::One(hand_val),
                _ => panic!("not real"),
            }
        }
    }
}
