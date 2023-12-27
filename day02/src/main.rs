use std::collections::HashMap;
use utils::read_input;
use anyhow::{bail, Result};
use phf::{phf_map};
use regex::Regex;

static DESIRED_CUBES: phf::Map<&'static str, &'static u32> = phf_map! {
    "red" => &12,
    "green" => &13,
    "blue" => &14,
};

fn main() -> Result<()> {
    let input = read_input()?;

    let p1_result = part_one(&input)?;
    let p2_result = part_one(&input)?;

    println!("P1 Result: {}", p1_result);
    println!("P2 Result: {}", p2_result);

    Ok(())
}

/*

Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

 */

#[derive(Debug)]
struct Game {
    id: u32,
    max_cubes: HashMap<String, u32>,
}

fn part_one(input: &String) -> Result<u32> {
    let mut games: Vec<Game> = Vec::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        let id = parse_game_id(line)?;
        let parts: Vec<&str> = line.split(": ").collect();
        let sets: Vec<&str> = parts[1].split("; ").collect();
        let max_cubes = parse_sets(sets);
        let game = Game {
            id,
            max_cubes,
        };
        games.push(game);
    }

    println!("{:?}", games);

    Ok(0)
}

fn parse_sets(sets: Vec<&str>) -> HashMap<String, u32> {
    let max_cubes = HashMap::<String, u32>::new();

    return max_cubes;
}

fn parse_game_id(line: &str) -> Result<u32> {
    let re = Regex::new(r"Game (\d+)")?;
    let Some(caps) = re.captures(line) else {
        bail!("unable to find game id from line");
    };
    let id = caps[1].parse::<u32>()?;
    Ok(id)
}