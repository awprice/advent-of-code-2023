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

    let games = parse_games(&input)?;

    let p1_result = part_one(&games);
    let p2_result = part_two(&games);

    println!("P1 Result: {}", p1_result);
    println!("P2 Result: {}", p2_result);

    Ok(())
}

#[derive(Debug)]
struct Game {
    id: u32,
    cubes: HashMap<String, u32>,
}

fn part_one(games: &Vec<Game>) -> u32 {
    let mut sum = 0;

    for game in games {
        let mut possible = true;

        for (colour, max) in DESIRED_CUBES.entries() {
            if game.cubes.get(*colour).unwrap_or(&0) > *max {
                possible = false;
                break;
            }
        }

        if possible {
            sum += game.id;
        }
    }

    sum
}

fn part_two(games: &Vec<Game>) -> u32 {
    let mut sum = 0;

    for game in games {
        let mut power_sum = 0;
        let values: Vec<&u32> = game.cubes.values().collect();
        for v in values {
            if power_sum == 0 {
                power_sum = *v;
            } else {
                power_sum *= v;
            }
        }

        sum += power_sum;
    }

    sum
}

fn parse_games(input: &str) -> Result<Vec<Game>> {
    let mut games: Vec<Game> = Vec::new();
    for line in input.lines() {
        let id = parse_game_id(line)?;
        let parts: Vec<&str> = line.split(": ").collect();
        let sets: Vec<&str> = parts[1].split("; ").collect();
        let cubes = parse_sets(sets)?;
        games.push(Game {
            id,
            cubes,
        });
    }

    Ok(games)
}

fn parse_sets(sets: Vec<&str>) -> Result<HashMap<String, u32>> {
    let mut cubes = HashMap::<String, u32>::new();
    let re = Regex::new(r"^(\d+) ([a-z]+)$")?;

    for set in sets {
        let parts: Vec<&str> = set.split(", ").collect();
        for cube in parts {
            let Some(caps) = re.captures(cube) else {
                bail!("unable to match num and colour of cubes");
            };
            let num= caps[1].parse::<u32>()?;
            let name = &caps[2];
            let v = cubes.entry(name.to_string()).or_insert(num);
            if num > *v {
                *v = num;
            }
        }
    }

    Ok(cubes)
}

fn parse_game_id(line: &str) -> Result<u32> {
    let re = Regex::new(r"Game (\d+)")?;
    let Some(caps) = re.captures(line) else {
        bail!("unable to find game id from line");
    };
    let id = caps[1].parse::<u32>()?;
    Ok(id)
}