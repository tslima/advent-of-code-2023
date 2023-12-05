use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("ERRO");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("ERRO");

    let result = engine_parts(&contents);

    println!("Etapa 1: {}", result);
}

#[test]
fn test_etapa1() {
    let input_text = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    let result = engine_parts(input_text);

    assert_eq!(result, 4361);
}

struct Coords {
    x: i32,
    y: i32,
}

fn engine_parts(input_text: &str) -> u32 {
    const RADIX: u32 = 10;
    let mut total = 0;
    let mut matrix_char = Vec::new();

    for line in input_text.lines() {
        let letters: Vec<char> = line.chars().collect();
        matrix_char.push(letters);
    }

    for i in 0..matrix_char.len() {
        let mut last_digit = 0;
        let mut found_symbol = false;

        for j in 0..matrix_char[i].len() {
            if matrix_char[i][j].is_digit(RADIX) {
                last_digit = last_digit * 10 + matrix_char[i][j].to_digit(RADIX).unwrap();

                let mut check_i = Vec::new();
                check_i.push(i);
                if i >= 1 {
                    check_i.push(i - 1);
                }

                if i < matrix_char.len() - 1 {
                    check_i.push(i + 1);
                }

                let mut check_j = Vec::new();
                check_j.push(j);
                if j >= 1 {
                    check_j.push(j - 1);
                }

                if j < matrix_char[i].len() - 1 {
                    check_j.push(j + 1);
                }
                for item_check_i in check_i {
                    for item_check_j in &check_j {
                        found_symbol |= !matrix_char[item_check_i][*item_check_j].is_digit(RADIX)
                            && matrix_char[item_check_i][*item_check_j] != '.';
                    }
                }
            } else {
                if found_symbol && last_digit > 0 {
                    total += last_digit;
                }
                last_digit = 0;
                found_symbol = false;
            }
        }
        if found_symbol {
            total += last_digit;
        }
    }

    total
}
