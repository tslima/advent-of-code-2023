use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

#[test]
fn test_1() {
    let text = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    assert_eq!(calibration_value(text), 142);
}

#[test]
fn teste_2() {
    let text = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    assert_eq!(calibration_value(text), 281);
}

fn main() {
    let mut file = File::open("input.txt").expect("ERRO");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("ERRO");
    let etapa_1 = calibration_value(&contents);

    println!("Etapa 1: {}", etapa_1);

    let mut file = File::open("input2.txt").expect("ERRO");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("ERRO");
    let etapa_2 = calibration_value(&contents);

    println!("Etapa 2: {}", etapa_2);
}

fn calibration_value(text: &str) -> u32 {
    const RADIX: u32 = 10;
    let re = Regex::new(r"[A-z]").unwrap();
    let re_text = text
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");
    let filter_text = re.replace_all(&re_text, "");

    let lines = filter_text.lines();

    let mut total = 0;
    for line in lines {
        if line.len() > 0 {
            let first: u32 = line.chars().next().unwrap().to_digit(RADIX).unwrap();
            let last: u32 = line.chars().last().unwrap().to_digit(RADIX).unwrap();
            total += first * 10 + last;
        }
    }
    total
}
