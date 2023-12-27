use utils::read_input;
use anyhow::Result;

fn main() -> Result<()> {
    let input = read_input()?;

    let p1_result = part_one(&input)?;
    let p2_result = part_one(&replace_digit_words(input))?;

    println!("P1 Result: {}", p1_result);
    println!("P2 Result: {}", p2_result);

    Ok(())
}

fn part_one(input: &str) -> Result<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let digits: Vec<u32> = line
            .chars()
            .filter_map(|a| a.to_digit(10))
            .collect();

        if digits.is_empty() {
            continue;
        }

        let line_num = format!(
            "{}{}",
            digits[0],
            digits[digits.len() - 1],
        ).parse::<u32>()?;
        sum += line_num;
    }
    Ok(sum)
}

fn replace_digit_words(input: String) -> String {
    input
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "th3ee")
        .replace("four", "fo4r")
        .replace("five", "fi5e")
        .replace("six", "s6x")
        .replace("seven", "se7en")
        .replace("eight", "ei8ht")
        .replace("nine", "ni9e")
}