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

pub struct Card {
    id: usize,
    my_numbers: Vec<usize>,
    winning_numbers: Vec<usize>,
}

fn parse_card(i: &str) -> IResult<&str, Card> {
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
    .map(|(i, (_, _, id, _, _, numbers, _, matching))| {
        (
            i,
            Card {
                id: id.parse().unwrap(),
                my_numbers: numbers.into_iter().map(|v| v.parse().unwrap()).collect(),
                winning_numbers: matching.into_iter().map(|v| v.parse().unwrap()).collect(),
            },
        )
    })
}

pub fn main() {
    let mut cards = Vec::new();
    for line in INPUT.lines() {
        if let Ok((_rest, card_line)) = all_consuming(parse_card)(line).finish() {
            cards.push(card_line);
        }
    }

    let mut won_copies = vec![1; cards.len()];

    let mut sum = 0;
    let mut matching_set = HashSet::new();
    for card in cards {
        let current_index = card.id - 1;

        if won_copies[current_index] == 0 {
            break;
        }

        matching_set.clear();
        matching_set.extend(card.winning_numbers.into_iter());

        let mut match_count = 0;
        for w in card.my_numbers {
            if matching_set.contains(&w) {
                match_count += 1;
            }
        }

        for i in 0..match_count {
            let index = current_index + i + 1;
            if index < won_copies.len() {
                won_copies[index] += won_copies[current_index];
            }
        }

        sum += won_copies[current_index];
    }

    println!("The sum of the winnings is {sum}");
}
