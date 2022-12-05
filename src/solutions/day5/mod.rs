use std::{
    cell::RefCell,
    fs::File,
    io::{BufRead, BufReader},
};

pub(crate) fn day5(data: BufReader<File>) {
    // Hard coding the input cause idk how to parse columns
    let col1 = RefCell::new(vec!['G', 'F', 'V', 'H', 'P', 'S']);
    let col2 = RefCell::new(vec!['G', 'J', 'F', 'B', 'V', 'D', 'Z', 'M']);
    let col3 = RefCell::new(vec!['G', 'M', 'L', 'J', 'N']);
    let col4 = RefCell::new(vec!['N', 'G', 'Z', 'V', 'D', 'W', 'P']);
    let col5 = RefCell::new(vec!['V', 'R', 'C', 'B']);
    let col6 = RefCell::new(vec!['V', 'R', 'S', 'M', 'P', 'W', 'L', 'Z']);
    let col7 = RefCell::new(vec!['T', 'H', 'P']);
    let col8 = RefCell::new(vec!['Q', 'R', 'S', 'N', 'C', 'H', 'Z', 'V']);
    let col9 = RefCell::new(vec!['F', 'L', 'G', 'P', 'V', 'Q', 'J']);

    let execute_part = std::env::args().nth(2).unwrap().parse::<u8>().unwrap();

    for line in data.lines() {
        let line = line.unwrap();
        let input = line.split_whitespace().collect::<Vec<&str>>();

        if input.is_empty() || input[0] != "move" {
            continue;
        }

        // Get only numeric values from the string
        let input: Vec<u32> = input.iter().filter_map(|x| x.parse::<u32>().ok()).collect();

        let num_blocks = input[0];
        let from = match input[1] {
            1 => &col1,
            2 => &col2,
            3 => &col3,
            4 => &col4,
            5 => &col5,
            6 => &col6,
            7 => &col7,
            8 => &col8,
            9 => &col9,
            _ => unreachable!(),
        };
        let to = match input[2] {
            1 => &col1,
            2 => &col2,
            3 => &col3,
            4 => &col4,
            5 => &col5,
            6 => &col6,
            7 => &col7,
            8 => &col8,
            9 => &col9,
            _ => unreachable!(),
        };

        match execute_part {
            // Part 1
            // Move num_blocks from from to to
            1 => execute_part1(num_blocks, from, to),
            // Part 2
            // Move num_blocks from from to to while preserving position
            2 => execute_part2(num_blocks, from, to),
            _ => unreachable!(),
        }
    }

    // Get the top boxes from each column;
    let top_boxes = &format!(
        "{}{}{}{}{}{}{}{}{}",
        col1.borrow()[col1.borrow().len() - 1],
        col2.borrow()[col2.borrow().len() - 1],
        col3.borrow()[col3.borrow().len() - 1],
        col4.borrow()[col4.borrow().len() - 1],
        col5.borrow()[col5.borrow().len() - 1],
        col6.borrow()[col6.borrow().len() - 1],
        col7.borrow()[col7.borrow().len() - 1],
        col8.borrow()[col8.borrow().len() - 1],
        col9.borrow()[col9.borrow().len() - 1]
    );
    println!("Top Boxes: {}", top_boxes);
}

fn execute_part1(num_blocks: u32, from: &RefCell<Vec<char>>, to: &RefCell<Vec<char>>) {
    for _ in 0..num_blocks {
        let block = from.borrow_mut().pop().unwrap();
        to.borrow_mut().push(block);
    }
}

fn execute_part2(num_blocks: u32, from: &RefCell<Vec<char>>, to: &RefCell<Vec<char>>) {
    let mut temp = vec![];
    for _ in 0..num_blocks {
        temp.push(from.borrow_mut().pop().unwrap());
    }
    temp.reverse();
    for block in temp {
        to.borrow_mut().push(block);
    }
}
