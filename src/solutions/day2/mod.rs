use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub(crate) fn day2(data: BufReader<File>) {
    let mut choices: HashMap<char, u32> = HashMap::new();
    let mut outcomes: HashMap<&str, u32> = HashMap::new();
    let mut total_score: u32 = 0;
    let mut strat_score: u32 = 0;

    choices.insert('X', 1);
    choices.insert('Y', 2);
    choices.insert('Z', 3);

    outcomes.insert("Lose", 0);
    outcomes.insert("Draw", 3);
    outcomes.insert("Win", 6);

    for line in data.lines() {
        let line = line.unwrap();
        let round: Vec<&str> = line.split_whitespace().collect();

        // Part 1
        total_score += execute_part1(&round, &choices, &outcomes);

        // Part 2
        strat_score += execute_part2(&round, &choices, &outcomes);
    }
    println!("Total score: {total_score}");
    println!("Strategy score: {strat_score}");
}

fn execute_part1(
    round: &[&str],
    choices: &HashMap<char, u32>,
    outcomes: &HashMap<&str, u32>,
) -> u32 {
    match round[0].chars().next().unwrap() {
        'A' => match round[1].chars().next().unwrap() {
            'X' => choices[&'X'] + outcomes["Draw"],
            'Y' => choices[&'Y'] + outcomes["Win"],
            'Z' => choices[&'Z'] + outcomes["Lose"],
            _ => unreachable!(),
        },
        'B' => match round[1].chars().next().unwrap() {
            'Y' => choices[&'Y'] + outcomes["Draw"],
            'Z' => choices[&'Z'] + outcomes["Win"],
            'X' => choices[&'X'] + outcomes["Lose"],
            _ => unreachable!(),
        },
        'C' => match round[1].chars().next().unwrap() {
            'X' => choices[&'X'] + outcomes["Win"],
            'Y' => choices[&'Y'] + outcomes["Lose"],
            'Z' => choices[&'Z'] + outcomes["Draw"],
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn execute_part2(
    round: &[&str],
    choices: &HashMap<char, u32>,
    outcomes: &HashMap<&str, u32>,
) -> u32 {
    match round[1].chars().next().unwrap() {
        // Need to lose
        'X' => match round[0].chars().next().unwrap() {
            'A' => choices[&'Z'] + outcomes["Lose"],
            'B' => choices[&'X'] + outcomes["Lose"],
            'C' => choices[&'Y'] + outcomes["Lose"],
            _ => unreachable!(),
        },
        // Need to draw
        'Y' => match round[0].chars().next().unwrap() {
            'A' => choices[&'X'] + outcomes["Draw"],
            'B' => choices[&'Y'] + outcomes["Draw"],
            'C' => choices[&'Z'] + outcomes["Draw"],
            _ => unreachable!(),
        },
        // Need to win
        'Z' => match round[0].chars().next().unwrap() {
            'A' => choices[&'Y'] + outcomes["Win"],
            'B' => choices[&'Z'] + outcomes["Win"],
            'C' => choices[&'X'] + outcomes["Win"],
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
