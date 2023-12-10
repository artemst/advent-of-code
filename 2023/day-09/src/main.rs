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

    for line in reader.lines() {
        let nums: Vec<i32> = line?.split(' ').map(|e| e.parse().unwrap()).collect();
        result_for_task_1 += get_value_1(&nums);
        result_for_task_2 += get_value_2(&nums);
    }

    println!("[1] Result for task 1 = {result_for_task_1}");
    println!("[2] Result for task 2 = {result_for_task_2}");

    Ok(())
}

fn get_value_1(nums: &[i32]) -> i32 {
    if nums.iter().filter(|&&el| el != 0).count() == 0 {
        return 0;
    }
    let mut next = Vec::new();
    for i in 1..nums.len() {
        next.push(nums[i] - nums[i - 1]);
    }
    nums.last().unwrap() + get_value_1(&next)
}

fn get_value_2(nums: &[i32]) -> i32 {
    if nums.iter().filter(|&&el| el != 0).count() == 0 {
        return 0;
    }
    let mut next = Vec::new();
    for i in 1..nums.len() {
        next.push(nums[i] - nums[i - 1]);
    }
    nums[0] - get_value_2(&next)
}
