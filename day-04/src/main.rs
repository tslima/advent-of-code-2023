use std::collections::HashSet;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("ERRO");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("ERRO");
    let e1 = winning_numbers(&contents);

    println!("Etapa 1: {}", e1);

    let e2 = copie_numbers(&contents);
    println!("Etapa 2: {}", e2);
}

#[test]
fn test_etapa_1() {
    let input_text = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let result = winning_numbers(input_text);

    assert_eq!(result, 13);
}

fn winning_numbers(input_text: &str) -> i32 {
    let mut total = 0;
    const BASE: i32 = 2;
    for line in input_text.lines() {
        let mut numbers = line.split(':');
        numbers.next();
        let mut numbers = numbers.next().unwrap().split("|");

        let w_numbers: HashSet<i32> = numbers
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect();

        let g_numbers: HashSet<i32> = numbers
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect();

        let w_g_numbers: Vec<&i32> = g_numbers.intersection(&w_numbers).collect();
        if w_g_numbers.len() > 0 {
            total += BASE.pow(w_g_numbers.len() as u32 - 1);
        }
    }
    total
}

#[test]
fn test_etapa_2() {
    let input_text = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let result = copie_numbers(input_text);

    assert_eq!(result, 30);
}

fn copie_numbers(input_text: &str) -> i32 {
    let mut total = 0;
    let mut copies = [1 as i32; 500];
    let mut max_row = 0;
    for (index, line) in input_text.lines().enumerate() {
        max_row = index;
        let mut numbers = line.split(':');
        numbers.next();
        let mut numbers = numbers.next().unwrap().split("|");

        let w_numbers: HashSet<i32> = numbers
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect();

        let g_numbers: HashSet<i32> = numbers
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect();

        let w_g_numbers: Vec<&i32> = g_numbers.intersection(&w_numbers).collect();

        for i in index + 1..=index + w_g_numbers.len() {
            copies[i] = copies[i] + copies[index];
        }
    }

    for i in 0..=max_row {
        total += copies[i];
    }
    total
}
