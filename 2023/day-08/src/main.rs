use core::panic;
use std::{collections::HashMap, io};

use num::integer::lcm;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input_filename = if args.get(1).is_some_and(|arg| arg.starts_with("-d")) {
        format!("input-test{}.txt", &args.get(1).unwrap()[2..])
    } else {
        "input.txt".to_string()
    };

    let file = std::fs::read_to_string(input_filename)?;
    let (instructions, map) = file.split_once("\n\n").unwrap();
    let instructions: Vec<Direction> = instructions
        .trim()
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            dir => panic!("Unknown direction: {dir}"),
        })
        .collect();

    let map = map
        .trim()
        .split('\n')
        .map(|row| row.split_once(" = ").unwrap())
        .map(|(left, right)| (left, right[1..9].split_once(", ").unwrap()))
        .fold(HashMap::new(), |mut acc, (key, val)| {
            acc.insert(key, val);
            acc
        });

    if map.contains_key("AAA") {
        let result_for_task_1 = get_value_for_task_1(&instructions, &map);
        println!("[1] Result for task 1 = {result_for_task_1}");
    }
    let result_for_task_2 = get_value_for_task_2(&instructions, &map);
    println!("[2] Result for task 2 = {result_for_task_2}");

    Ok(())
}

fn get_value_for_task_1(instructions: &[Direction], map: &HashMap<&str, (&str, &str)>) -> usize {
    let mut last_code = "AAA";
    for (counter, i) in instructions.iter().cycle().enumerate() {
        last_code = match i {
            Direction::Left => map[last_code].0,
            Direction::Right => map[last_code].1,
        };
        // println!("counter: {counter}; dir: {i:?}, code: {code}");
        if last_code == "ZZZ" {
            return counter + 1;
        }
    }
    0
}

fn get_value_for_task_2(instructions: &[Direction], map: &HashMap<&str, (&str, &str)>) -> usize {
    let starts: Vec<&str> = map.keys().cloned().filter(|k| k.ends_with('A')).collect();
    starts
        .iter()
        .map(|&k| {
            let mut last_code = k;
            for (counter, i) in instructions.iter().cycle().enumerate() {
                last_code = match i {
                    Direction::Left => map[last_code].0,
                    Direction::Right => map[last_code].1,
                };
                if last_code.ends_with('Z') {
                    return counter + 1;
                }
            }
            0
        })
        .fold(1, lcm) // calculate Least Common Multiply for all results
}
