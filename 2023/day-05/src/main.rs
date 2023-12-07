use std::io::{self};

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input_filename = if args.get(1).is_some_and(|arg| arg == "-d") {
        "input-test.txt"
    } else {
        "input.txt"
    };

    let file = std::fs::read_to_string(input_filename)?;
    let data = Data::from(&file);

    let result_for_task_1 = data
        .seeds
        .iter()
        .map(|s| get_seed_location(s, &data.rules))
        .min()
        .unwrap();

    let mut result_for_task_2 = 0;

    let mut seeds_iter = data.seeds.iter();
    while let (Some(&seed_start), Some(&seed_len)) = (seeds_iter.next(), seeds_iter.next()) {
        for seed in seed_start..seed_start + seed_len {
            let result = get_seed_location(&seed, &data.rules);
            result_for_task_2 = if result_for_task_2 == 0 {
                result
            } else {
                result_for_task_2.min(result)
            };
        }
    }

    println!("[1] Result for task 1 = {result_for_task_1}");
    println!("[2] Result for task 2 = {result_for_task_2}");

    Ok(())
}

fn get_seed_location(seed: &u64, rules: &Vec<Vec<Rule>>) -> u64 {
    let mut result = *seed;
    for rule_set in rules {
        let found = rule_set.iter().find_map(|r| {
            if result >= r.src && result < r.src + r.len {
                Some(r.dest + result - r.src)
            } else {
                None
            }
        });
        // println!("{result} => {:?}", found);
        result = found.unwrap_or(result)
    }
    result
}

#[derive(Debug)]
struct Data {
    seeds: Vec<u64>,
    rules: Vec<Vec<Rule>>,
}

#[derive(Debug)]
struct Rule {
    dest: u64,
    src: u64,
    len: u64,
}

impl Data {
    fn from(str: &str) -> Self {
        let (seeds, rules) = str.split_once("\n\n").unwrap();
        let seeds = seeds
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .split(' ')
            .map(|s| s.parse().unwrap())
            .collect();

        // println!("Seeds: {:?}", seeds);
        let rules = rules
            .trim()
            .split("\n\n")
            .map(|step| {
                step.split_once(":\n")
                    .unwrap()
                    .1
                    .split('\n')
                    .map(Rule::from)
                    .collect::<Vec<Rule>>()
            })
            .collect();

        Self { seeds, rules }
    }
}

impl Rule {
    fn from(str: &str) -> Self {
        let arr = str
            .split(' ')
            .map(|e| e.parse().unwrap())
            .collect::<Vec<u64>>();
        Self {
            dest: arr[0],
            src: arr[1],
            len: arr[2],
        }
    }
}
