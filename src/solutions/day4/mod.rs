use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub(crate) fn day4(data: BufReader<File>) {
    let mut total_pairs = 0;
    let mut total_overlapping_pairs = 0;

    for line in data.lines() {
        let line = line.unwrap();
        let range: Vec<&str> = line.split(['-', ',']).collect();
        // Convert the range to a vector of integers.
        let range: Vec<i32> = range.iter().map(|x| x.parse::<i32>().unwrap()).collect();

        // Check to see if the first 2 numbers are within the range of
        // the 3rd and 4th numbers and vice versa
        if (range[0] >= range[2]
            && range[0] <= range[3]
            && range[1] >= range[2]
            && range[1] <= range[3])
            || (range[2] >= range[0]
                && range[2] <= range[1]
                && range[3] >= range[0]
                && range[3] <= range[1])
        {
            total_pairs += 1;
        }

        // Find the overlapping pairs
        if (range[0] >= range[2]
            && range[0] <= range[3])
            || (range[1] >= range[2]
            && range[1] <= range[3])
            || (range[2] >= range[0]
            && range[2] <= range[1])
            || (range[3] >= range[0]
            && range[3] <= range[1])
        {
            total_overlapping_pairs += 1;
        }
    }
    println!("Total pairs: {}", total_pairs);
    println!("Total overlapping pairs: {}", total_overlapping_pairs);
}
