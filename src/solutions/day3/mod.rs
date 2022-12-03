use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

pub(crate) fn day3(data: BufReader<File>) {
    let mut total_priority_p1: u32 = 0;
    let mut total_priority_p2: u32 = 0;
    let lines: Vec<_> = data.lines().collect();

    // Part 1
    for line in &lines {
        total_priority_p1 += execute_part1(line);
    }

    // Part 2
    // Find the common character from pairs of 3 lines
    let mut i = 0;
    while i < lines.len() {
        let line1 = &lines[i];
        let line2 = &lines[i + 1];
        let line3 = &lines[i + 2];
        total_priority_p2 += execute_part2(line1, line2, line3);
        i += 3;
    }

    println!("Part 1: Total priority: {}", total_priority_p1);
    println!("Part 2: Total priority: {}", total_priority_p2);
}

fn execute_part1(line: &Result<String, Error>) -> u32 {
    let chars: Vec<char> = line.as_ref().unwrap().chars().collect();
    // Go through first half of vector and see if there are any similar character in the second half.
    // If there are, then we have a match.
    for (i, c) in chars.iter().enumerate() {
        if i < chars.len() / 2 {
            let second_half = &chars[chars.len() / 2..];
            if second_half.contains(c) {
                return get_priority(c);
            }
        }
    }
    unreachable!()
}

fn execute_part2(
    line1: &Result<String, Error>,
    line2: &Result<String, Error>,
    line3: &Result<String, Error>,
) -> u32 {
    let chars1: Vec<char> = line1.as_ref().unwrap().chars().collect();

    for c in chars1 {
        if line2.as_ref().unwrap().contains(c) && line3.as_ref().unwrap().contains(c) {
            return get_priority(&c);
        }
    }
    unreachable!()
}

fn get_priority(character: &char) -> u32 {
    if ('A' as u32 <= *character as u32) && (*character as u32 <= 'Z' as u32) {
        *character as u32 - 'A' as u32 + 27
    } else {
        *character as u32 - 'a' as u32 + 1
    }
}
