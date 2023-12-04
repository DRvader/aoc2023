use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace1},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{terminated, tuple},
    Finish, IResult,
};

const INPUT: &'static str = include_str!("input.txt");

/// Return the number of matches
fn parse_card(i: &str) -> IResult<&str, usize> {
    let mut matching_set: HashSet<usize> = HashSet::new();

    tuple((
        tag("Card"),
        multispace1,
        digit1,
        tag(":"),
        multispace1,
        terminated(
            separated_list1(multispace1, digit1),
            tuple((multispace1, tag("|"))),
        ),
        multispace1,
        separated_list1(multispace1, digit1),
    ))(i)
    .map(|(i, (_, _, _id, _, _, numbers, _, matching))| {
        matching_set.clear();

        matching_set.extend(matching.into_iter().map(|v| v.parse::<usize>().unwrap()));

        let mut match_count = 0;
        for number in numbers.into_iter().map(|v| v.parse().unwrap()) {
            if matching_set.contains(&number) {
                match_count += 1;
            }
        }

        (i, match_count)
    })
}

pub fn main() {
    let mut sum = 0;
    for line in INPUT.lines() {
        if let Ok((_rest, match_count)) = all_consuming(parse_card)(line).finish() {
            if match_count > 0 {
                sum += 1 << (match_count - 1);
            }
        }
    }

    println!("The sum of the winnings is {sum}");
}
