use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

#[derive(Debug, PartialEq, Default)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut result_for_task_1 = 0;
    let mut result_for_task_2 = 0;
    for line in reader.lines() {
        let game = Game::from(&line?);
        result_for_task_1 += get_game_value_1(&game);
        result_for_task_2 += get_game_value_2(&game);
    }
    println!("[1] Result for task 1 = {result_for_task_1}");
    println!("[2] Result for task 2 = {result_for_task_2}");

    Ok(())
}

fn get_game_value_1(game: &Game) -> u32 {
    let impossible_game = game
        .rounds
        .iter()
        .any(|r| r.red > 12 || r.green > 13 || r.blue > 14);
    if impossible_game {
        0
    } else {
        game.id
    }
}

fn get_game_value_2(game: &Game) -> u32 {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;
    for round in &game.rounds {
        if round.red > max_red {
            max_red = round.red;
        }
        if round.green > max_green {
            max_green = round.green;
        }
        if round.blue > max_blue {
            max_blue = round.blue;
        }
    }
    max_red * max_green * max_blue
}

impl Game {
    fn from(s: &str) -> Self {
        let (game, game_data) = s.split_once(':').unwrap();
        let (_, game_id) = game.split_once(' ').unwrap();
        let rounds: Vec<Round> = game_data
            .split(';')
            .map(|entry| Round::from(entry.trim()))
            .collect();
        Self {
            id: game_id.parse().ok().unwrap(),
            rounds,
        }
    }
}

impl Round {
    fn from(s: &str) -> Self {
        let mut result = Round::default();
        let all_cubes: Vec<&str> = s.split(',').map(|c| c.trim()).collect();
        for colored_cubes in all_cubes {
            let (number, color) = colored_cubes.split_once(' ').unwrap();
            match color {
                "red" => result.red = number.parse().ok().unwrap(),
                "green" => result.green = number.parse().ok().unwrap(),
                "blue" => result.blue = number.parse().ok().unwrap(),
                _ => panic!("Unknown color '{color}'"),
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::HashMap;

    fn r(red: u32, green: u32, blue: u32) -> Round {
        Round { red, green, blue }
    }

    #[test]
    fn test_parsing_line_to_game_struct() {
        let td = HashMap::from([
            (
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
                Game {
                    id: 1,
                    rounds: vec![r(4, 0, 3), r(1, 2, 6), r(0, 2, 0)],
                },
            ),
            (
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
                Game {
                    id: 2,
                    rounds: vec![r(0, 2, 1), r(1, 3, 4), r(0, 1, 1)],
                },
            ),
            (
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                Game {
                    id: 3,
                    rounds: vec![r(20, 8, 6), r(4, 13, 5), r(1, 5, 0)],
                },
            ),
            (
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                Game {
                    id: 4,
                    rounds: vec![r(3, 1, 6), r(6, 3, 0), r(14, 3, 15)],
                },
            ),
            (
                "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
                Game {
                    id: 5,
                    rounds: vec![r(6, 3, 1), r(1, 2, 2)],
                },
            ),
        ]);

        for (test, expected) in td {
            assert_eq!(Game::from(test), expected);
        }
    }
}
