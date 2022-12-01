use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub(crate) fn day1(data: BufReader<File>) {
    let mut calories: Vec<i32> = vec![];
    let mut count = 0;
    let mut total = 0;

    for line in data.lines() {
        // If the line is a number, add it to the count
        match line.unwrap().parse::<i32>() {
            Ok(num) => count += num,
            Err(_) => {
                // If it's an empty line, add the count to the total and reset the count
                calories.push(count);
                count = 0;
            }
        }
    }

    println!("\n====== Advent of Code Day 1 ======");
    for i in 0..3 {
        // Get the max calories and its index
        let (index, value) = calories
            .iter()
            .enumerate()
            .max_by_key(|&(_, &v)| v)
            .unwrap();
        total += value;
        println!("{i}: Max calories {value} at index {index}");
        // Remove the current max calories to get the next max.
        calories.remove(index);
    }
    println!("Total calories: {}", total);
    println!("==================================");
}
