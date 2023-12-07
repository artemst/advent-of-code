use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input_filename = if args.get(1).is_some_and(|arg| arg == "-d") {
        "input-test.txt"
    } else {
        "input.txt"
    };

    let file = std::fs::read_to_string(input_filename)?;
    let data = file.split_once('\n').unwrap();

    let result_for_task_1 = get_value_for_task_1(data);
    let result_for_task_2 = get_value_for_task_2(data);

    println!("[1] Result for task 1 = {result_for_task_1}");
    println!("[2] Result for task 2 = {result_for_task_2}");

    Ok(())
}

fn get_value_for_task_1((time, distance): (&str, &str)) -> u64 {
    time.split(' ')
        .filter_map(|e| e.parse().ok())
        .zip(distance.trim().split(' ').filter_map(|e| e.parse().ok()))
        .map(|(time, distance)| count_winning_ways(time, distance))
        .product::<u64>()
}

fn get_value_for_task_2((time, distance): (&str, &str)) -> u64 {
    let time = time.split_once(':').unwrap().1.replace(' ', "");
    let distance = distance.trim().split_once(':').unwrap().1.replace(' ', "");
    count_winning_ways(time.parse().unwrap(), distance.parse().unwrap())
}

fn count_winning_ways(time: u64, distance: u64) -> u64 {
    let mut counter = 0;
    for i in 1..time {
        if i * (time - i) > distance {
            counter += 1;
        }
    }
    counter
}
