use std::num::ParseIntError;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_till1},
    character::complete::{alpha0, anychar, digit1, newline, one_of},
    combinator::{map, map_res, opt, value},
    error::{Error, ErrorKind},
    multi::{many0, many1, many_till, separated_list1},
    number, Err, IResult, Parser,
};

advent_of_code::solution!(1);

fn combine_first_and_last_digit_chars(input: &Vec<char>) -> Result<u32, ParseIntError> {
    let first = input.first().unwrap();
    let last = input.last().unwrap();
    let number = format!("{first}{last}");
    number.parse::<u32>()
}

fn combine_first_and_last_digit_u32(input: &Vec<u32>) -> Result<u32, ParseIntError> {
    let first = input.first().unwrap();
    let last = input.last().unwrap();
    Ok(first * 10 + last)
}

#[derive(Debug, Clone)]
enum ParsedDigit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Number(u32),
}
impl ParsedDigit {
    fn value(&self) -> u32 {
        match self {
            ParsedDigit::One => 1,
            ParsedDigit::Two => 2,
            ParsedDigit::Three => 3,
            ParsedDigit::Four => 4,
            ParsedDigit::Five => 5,
            ParsedDigit::Six => 6,
            ParsedDigit::Seven => 7,
            ParsedDigit::Eight => 8,
            ParsedDigit::Nine => 9,
            ParsedDigit::Number(n) => *n,
        }
    }
}

fn parser_rules(input: &str) -> IResult<&str, Option<ParsedDigit>> {
    opt(alt((
        value(ParsedDigit::One, tag("one")),
        value(ParsedDigit::Two, tag("two")),
        value(ParsedDigit::Three, tag("three")),
        value(ParsedDigit::Four, tag("four")),
        value(ParsedDigit::Five, tag("five")),
        value(ParsedDigit::Six, tag("six")),
        value(ParsedDigit::Seven, tag("seven")),
        value(ParsedDigit::Eight, tag("eight")),
        value(ParsedDigit::Nine, tag("nine")),
        map_res(take(1usize), |digit: &str| {
            digit.parse::<u32>().map(ParsedDigit::Number)
        }),
    )))(input)
}

// parse_digits takes one charachter at a time from the input until
// it is able to parse a word or number digit, ignores the characters that
// weren't part of a word or digit, and returns a single flattened Vec<&str> of
// the result.
fn parse_digits(input: &str) -> IResult<&str, Vec<&str>> {
    let (rest, digit_vecs) = many1(map(
        many_till(take(1u8), parse_word_or_num_digit),
        |(_, word_or_num)| word_or_num,
    ))(input)?;
    Ok((rest, digit_vecs.into_iter().flatten().collect()))
}

fn parse_word_or_num_digit(input: &str) -> IResult<&str, Vec<&str>> {
    alt((parse_word_digit, map(digit1, |d| vec![d])))(input)
}

// Note that certain words can overlap, the only one I can see is:
// sevenine (correction - there are quite a few, see below).
// LEARN: avoid confirmation bias - finding one exception and thinking that's the
// missing piece, rather than looking exhaustively for all exceptions.
fn parse_word_digit(input: &str) -> IResult<&str, Vec<&str>> {
    map(
        alt((
            tag("oneight"),
            tag("one"),
            tag("twone"),
            tag("two"),
            tag("threeight"),
            tag("three"),
            tag("four"),
            tag("fiveight"),
            tag("five"),
            tag("six"),
            tag("sevenine"),
            tag("seven"),
            tag("eightwo"),
            tag("eighthree"),
            tag("eight"),
            tag("nineight"),
            tag("nine"),
        )),
        |word| match word {
            "oneight" => vec!["1", "8"],
            "one" => vec!["1"],
            "twone" => vec!["2", "1"],
            "two" => vec!["2"],
            "threeight" => vec!["3", "8"],
            "three" => vec!["3"],
            "four" => vec!["4"],
            "fiveight" => vec!["5", "8"],
            "five" => vec!["5"],
            "six" => vec!["6"],
            "sevenine" => vec!["7", "9"],
            "seven" => vec!["7"],
            "eighthree" => vec!["8", "3"],
            "eightwo" => vec!["8", "2"],
            "eight" => vec!["8"],
            "nineight" => vec!["9", "8"],
            "nine" => vec!["9"],
            _ => panic!("non digit word"),
        },
    )(input)
}

/// first_n_last extracts the first digit of the first number in
/// a vec of string numbers, together with the last digit of the
/// last number.
/// So for example, example: given the input vec!["24", "32", "98"],
/// the resulting calibration is "28".
fn first_n_last(multidigits: Vec<&str>) -> u32 {
    // First character of the first number string,
    // parsed base 10.
    let first = multidigits
        .first()
        .expect("must be at least one number on a line")
        .chars()
        .nth(0)
        .expect("must be at least one digit in a number")
        .to_digit(10)
        .expect("should be base 10 digit");
    // Last character of the last number string, parsed base 10
    let last = multidigits
        .last()
        .expect("must be at least one number on a line")
        .chars()
        .last()
        .expect("must be at least one digit in a number")
        .to_digit(10)
        .unwrap();
    first * 10 + last
}

pub fn parse_line(input: &str) -> IResult<&str, u32> {
    map(parse_digits, |digits: Vec<&str>| first_n_last(digits))(input)
}

fn parse(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .map(|line| {
            let (_, calibration) =
                parse_line(line).expect("each line should have a valid calibration");
            calibration
        })
        .sum();
    Some(sum)
}

// Create a nom parser that extracts the first and last digit of a line
// and then parses the two digits into a number.
// Then sum all the numbers.
pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let digits = line.chars().filter(|c| c.is_ascii_digit()).collect_vec();
        let number = combine_first_and_last_digit_chars(&digits).ok()?;
        sum += number;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }
    #[test]
    fn test_parse() {
        let input = "twopne3four";
        let sum = parse(input).unwrap();
        assert_eq!(sum, 24);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        dbg!(&result);
    }
}
