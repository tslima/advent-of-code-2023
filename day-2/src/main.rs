use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("ERRO");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("ERRO");
    let reds = 12;
    let blues = 14;
    let greens = 13;
    let etapa_1 = game_cubes(&contents, reds, greens, blues);

    println!("Etapa 1: {}", etapa_1);
}

#[test]
fn teste_01() {
    let input_games = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let reds = 12;
    let greens = 13;
    let blues = 14;
    let result = game_cubes(input_games, reds, greens, blues);

    assert_eq!(8, result);
}

fn game_cubes(input_games: &str, reds: i32, greens: i32, blues: i32) -> i32 {
    let mut total = 0;

    for line in input_games.lines() {
        let mut game_content = line.split(":");

        let game_index = game_content
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let mut valid_game = true;

        for game_set in game_content.next().unwrap().split(";") {
            let mut red_ct = 0;
            let mut green_ct = 0;
            let mut blue_ct = 0;

            for game_rolls in game_set.split(",") {
                let mut count_color = game_rolls.trim().split_whitespace();

                let count = count_color.next().unwrap().parse::<i32>().unwrap();
                let color = count_color.next().unwrap();

                if color == "red" {
                    red_ct += count;
                } else if color == "green" {
                    green_ct += count;
                } else if color == "blue" {
                    blue_ct += count;
                }
            }
            if red_ct > reds || green_ct > greens || blue_ct > blues {
                valid_game = false;
                break;
            }
        }
        if valid_game {
            total += game_index;
        }
    }

    total
}
