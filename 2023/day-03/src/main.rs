use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let mut lines = BufReader::new(file).lines();

    let mut result_for_task_1 = 0;
    let mut result_for_task_2 = 0;

    let mut prev = Line::from(&lines.next().unwrap()?); // init with 1st line
    let mut curr = Line::from(&lines.next().unwrap()?); // init with 2nd line

    // get values for the first line
    result_for_task_1 += get_line_value_1(&prev, vec![&curr]);
    result_for_task_2 += get_line_value_2(&prev, vec![&curr]);
    while let Some(Ok(next)) = lines.next() {
        let next = Line::from(&next);
        result_for_task_1 += get_line_value_1(&curr, vec![&prev, &next]);
        result_for_task_2 += get_line_value_2(&curr, vec![&prev, &next]);
        prev = curr;
        curr = next;
    }
    // get values for the last line
    result_for_task_1 += get_line_value_1(&curr, vec![&prev]);
    result_for_task_2 += get_line_value_2(&curr, vec![&prev]);

    println!("[1] Result for task 1 = {result_for_task_1}");
    println!("[2] Result for task 2 = {result_for_task_2}");

    Ok(())
}

type Number = u32;
type Position = usize;

#[derive(Debug, PartialEq)]
struct Line {
    nums: Vec<(Number, Position)>,
    symbols: Vec<Position>,
    gears: Vec<Position>,
}

impl Line {
    fn from(str: &str) -> Self {
        let (nums, symbols, gears) = get_positions(str);
        Self {
            nums,
            symbols,
            gears,
        }
    }
}

fn get_positions(line: &str) -> (Vec<(Number, Position)>, Vec<Position>, Vec<Position>) {
    let mut nums_pos: Vec<(Number, Position)> = Vec::new();
    let mut all_sym_pos: Vec<Position> = Vec::new();
    let mut gears_pos: Vec<Position> = Vec::new();

    let mut position: Position = 0;
    let mut num: String = "".to_string();
    for (pos, ch) in line.char_indices() {
        // symbol
        if ch != '.' && !ch.is_ascii_digit() {
            all_sym_pos.push(pos + 1);
        }
        // gear (asterisk)
        if ch == '*' {
            gears_pos.push(pos + 1);
        }
        if ch.is_ascii_digit() {
            if num.is_empty() {
                // start of number
                position = pos;
            }
            // number continues
            num.push(ch);
        } else if !num.is_empty() {
            // end of number
            nums_pos.push((num.parse().ok().unwrap(), position + 1));
            num = "".to_string();
        }
    }
    if !num.is_empty() {
        // number was at the end of line
        nums_pos.push((num.parse().ok().unwrap(), position + 1));
    }

    (nums_pos, all_sym_pos, gears_pos)
}

fn get_line_value_1(current: &Line, neighbours: Vec<&Line>) -> u32 {
    current
        .nums
        .iter()
        .map(|(num, pos)| (*num, *pos, num.to_string().len()))
        .filter(|(_, pos, num_len)| {
            [&neighbours[..], &[current]]
                .concat()
                .iter()
                .flat_map(|l| &l.symbols)
                .any(|sym_pos| *sym_pos >= (pos - 1) && *sym_pos <= pos + num_len)
        })
        .map(|(num, _, _)| num)
        .sum()
}

fn get_line_value_2(current: &Line, neighbours: Vec<&Line>) -> u32 {
    current
        .gears
        .iter()
        .filter_map(|&gear_pos| {
            let mut result: Vec<Number> = Vec::new();
            for (num, pos, num_len) in [&neighbours[..], &[current]]
                .concat()
                .iter()
                .flat_map(|l| &l.nums)
                .map(|(num, pos)| (*num, *pos, num.to_string().len()))
            {
                if gear_pos >= (pos - 1) && gear_pos <= pos + num_len {
                    // adjucent number
                    if result.len() == 2 {
                        // found 3rd number
                        return None;
                    }
                    result.push(num);
                }
            }
            if result.len() == 2 {
                Some((result[0], result[1]))
            } else {
                None
            }
        })
        .map(|(l, r)| l * r)
        .sum()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_parsig_line() {
        #[rustfmt::skip]
        let td = HashMap::from([
            ("467..11...", Line { nums: vec![(467, 1), (11, 6)], symbols: vec![], gears: vec![] }),
            ("...*......", Line { nums: vec![], symbols: vec![4], gears: vec![4] }),
            ("(........3", Line { nums: vec![(3, 10)], symbols: vec![1], gears: vec![] }),
            ("...$.*.*..", Line { nums: vec![], symbols: vec![4, 6, 8], gears: vec![6 ,8] }),
        ]);

        for (test, expected) in td {
            assert_eq!(Line::from(test), expected);
        }
    }

    #[test]
    fn test_get_line_value_1() {
        #[rustfmt::skip]
        let td = vec![
            ("467..114..", 467),
            ("...*......", 0),
            ("..35..633.", 668),
            ("......#...", 0),
            ("617*......", 617),
            (".....+.58.", 0),
            ("..592.....", 592),
            ("......755.", 755),
            ("...$.*....", 0),
            (".664.598..", 1262),
        ];

        assert_eq!(
            get_line_value_1(&Line::from(td[0].0), vec![&Line::from(td[1].0)]),
            td[0].1
        );
        assert_eq!(
            get_line_value_1(&Line::from(td[9].0), vec![&Line::from(td[8].0)]),
            td[9].1
        );
        for i in 1..9 {
            assert_eq!(
                get_line_value_1(
                    &Line::from(td[i].0),
                    vec![&Line::from(td[i - 1].0), &Line::from(td[i + 1].0)]
                ),
                td[i].1
            );
        }
    }

    #[test]
    fn test_get_line_value_2() {
        #[rustfmt::skip]
        let td = vec![
            ("467..114..", 0),
            ("...*......", 16345),
            ("...35*633.", 22155),
            ("......#...", 0),
            ("617*.5....", 0),
            (".....+.58.", 0),
            ("..59*.....", 0),
            (".......55.", 0),
            ("...$2*....", 0),
            (".664...8..", 0),
        ];

        assert_eq!(
            get_line_value_2(&Line::from(td[0].0), vec![&Line::from(td[1].0)]),
            td[0].1
        );
        assert_eq!(
            get_line_value_2(&Line::from(td[9].0), vec![&Line::from(td[8].0)]),
            td[9].1
        );
        for i in 1..9 {
            assert_eq!(
                get_line_value_2(
                    &Line::from(td[i].0),
                    vec![&Line::from(td[i - 1].0), &Line::from(td[i + 1].0)]
                ),
                td[i].1
            );
        }
    }
}
