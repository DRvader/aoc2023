use std::{iter::Sum, ops::AddAssign};

use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::all_consuming,
    multi::separated_list1, sequence::tuple, Finish, IResult,
};

const INPUT: &'static str = include_str!("input.txt");

#[derive(Default, Debug)]
pub struct Turn {
    red: usize,
    green: usize,
    blue: usize,
}

impl AddAssign<(TurnType, usize)> for Turn {
    fn add_assign(&mut self, (ty, count): (TurnType, usize)) {
        match ty {
            TurnType::Red => self.red += count,
            TurnType::Green => self.green += count,
            TurnType::Blue => self.blue += count,
        }
    }
}

impl Sum for Turn {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = Turn::default();
        for i in iter {
            sum.red += i.red;
            sum.green += i.green;
            sum.blue += i.blue;
        }

        sum
    }
}

pub enum TurnType {
    Red,
    Green,
    Blue,
}

impl From<&str> for TurnType {
    fn from(i: &str) -> Self {
        match i.to_lowercase().as_str() {
            "red" => TurnType::Red,
            "green" => TurnType::Green,
            "blue" => TurnType::Blue,
            _ => unimplemented!("no other turn types supported"),
        }
    }
}

fn parse_turns(i: &str) -> IResult<&str, Turn> {
    separated_list1(tag(", "), parse_turn)(i).map(|(i, turns)| {
        let mut sum = Turn::default();
        for turn in turns {
            sum += turn;
        }

        (i, sum)
    })
}

fn parse_turn(i: &str) -> IResult<&str, (TurnType, usize)> {
    tuple((
        digit1,
        tag(" "),
        alt((tag("red"), tag("green"), tag("blue"))),
    ))(i)
    .map(|(i, (value, _, turn_type))| (i, (turn_type.into(), value.parse().unwrap())))
}

pub struct Game {
    id: usize,
    turns: Vec<Turn>,
}

fn parse_game(i: &str) -> IResult<&str, Game> {
    tuple((
        tag("Game "),
        digit1,
        tag(": "),
        separated_list1(tag("; "), parse_turns),
    ))(i)
    .map(|(i, (_, id, _, v))| {
        (
            i,
            Game {
                id: id.parse().unwrap(),
                turns: v,
            },
        )
    })
}

pub fn main() {
    let mut games = vec![];
    for line in INPUT.lines() {
        if let Ok((_rest, game_line)) = all_consuming(parse_game)(line).finish() {
            games.push(game_line);
        }
    }

    let mut sum = 0;
    for game in games {
        let mut valid_game = true;
        for turn in game.turns {
            if turn.red > 12 || turn.green > 13 || turn.blue > 14 {
                valid_game = false;
                break;
            }
        }

        if valid_game {
            sum += game.id;
        }
    }

    println!("The sum of all possible games is {sum}");
}
