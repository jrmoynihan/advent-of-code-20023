use itertools::Itertools;
use std::{
    cmp::{max, min},
    collections::HashMap,
    string::ParseError,
};

advent_of_code::solution!(2);

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
enum CubeColor {
    Red,
    Green,
    Blue,
}
#[derive(Debug)]
struct Game {
    id: u32,
    cubes: HashMap<CubeColor, u32>,
}

fn parse_game(input: &str) -> Result<Game, Box<dyn std::error::Error>> {
    let id = input
        .split(":")
        .next()
        .ok_or("not found")?
        .split(" ")
        .last()
        .ok_or("not found")?;
    let cube_sets = input.split(":").last().ok_or("not found")?;
    let mut cubes = HashMap::new();
    cube_sets.split(";").for_each(|set| {
        set.split(",").for_each(|c| {
            if let Some((count, color)) = c.trim().split(" ").collect_tuple::<(&str, &str)>() {
                let color = match color {
                    "red" => CubeColor::Red,
                    "green" => CubeColor::Green,
                    "blue" => CubeColor::Blue,
                    _ => unreachable!(),
                };
                // Update the count for the color if this value is greater than the current count
                *cubes.entry(color.clone()).or_insert(0) = max(
                    *cubes.get(&color).unwrap_or(&0),
                    count.parse::<u32>().unwrap(),
                );
            }
        })
    });
    println!("Game:{}", id);
    println!("{:?}", cubes);
    let game = Game {
        id: id.parse().unwrap(),
        cubes,
    };
    Ok(game)
}

pub fn part_one(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .map(parse_game)
        .filter_ok(|game| {
            game.cubes.get(&CubeColor::Red).unwrap_or(&0) <= &12
                && game.cubes.get(&CubeColor::Green).unwrap_or(&0) <= &13
                && game.cubes.get(&CubeColor::Blue).unwrap_or(&0) <= &14
        })
        .map(|game| game.unwrap().id)
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .map(parse_game)
        .map(|game| game.unwrap().cubes.values().product::<u32>())
        .sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
