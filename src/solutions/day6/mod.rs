use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub(crate) fn day6(data: BufReader<File>) {
    // Get the first line in the file
    let line = data.lines().next().unwrap().unwrap();

    let mut alphabet: Vec<char> = vec![];

    println!(
        "Part 1 Result: {}",
        get_message_result(4, line.clone(), &mut alphabet)
    );
    println!(
        "Part 2 Result: {}",
        get_message_result(14, line, &mut alphabet)
    );
}

fn get_message_result(num_unique_chars: u8, line: String, alphabet: &mut Vec<char>) -> i32 {
    for (idx, c) in line.chars().enumerate() {
        if !alphabet.contains(&c) {
            alphabet.push(c);
        } else {
            // Remove characters till there are no more duplicates
            while alphabet.contains(&c) {
                alphabet.remove(0);
            }
            alphabet.push(c);
        }

        if alphabet.len() == num_unique_chars as usize {
            return idx as i32 + 1;
        }
    }
    unreachable!();
}
