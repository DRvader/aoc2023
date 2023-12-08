use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline, one_of, space0, multispace0},
    combinator::all_consuming,
    multi::{many1, separated_list1},
    sequence::{terminated, tuple},
    Finish, IResult,
};

const INPUT: &'static str = include_str!("input.txt");

/*
const INPUT: &'static str = r#"
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#;
*/

pub enum LR {
    Left,
    Right,
}

impl From<char> for LR {
    fn from(value: char) -> Self {
        match value {
            'L' => LR::Left,
            'R' => LR::Right,
            _ => unimplemented!(),
        }
    }
}

pub struct Instructions<'a> {
    directions: Vec<LR>,
    nodes: HashMap<&'a str, (&'a str, &'a str)>,
}

pub fn parse_node(i: &str) -> IResult<&str, (&str, (&str, &str))> {
    tuple((
        alphanumeric1,
        tag(" = ("),
        alphanumeric1,
        space0,
        tag(","),
        space0,
        alphanumeric1,
        space0,
        tag(")"),
    ))(i)
    .map(|(i, (key, _, left, _, _, _, right, _, _))| (i, (key, (left, right))))
}

pub fn parse_nodes(i: &str) -> IResult<&str, HashMap<&str, (&str, &str)>> {
    separated_list1(newline, parse_node)(i).map(|(i, v)| {
        let mut nodes = HashMap::with_capacity(v.len());
        for (key, (left, right)) in v {
            nodes.insert(key, (left, right));
        }
        (i, nodes)
    })
}

pub fn parse_instructions(i: &str) -> IResult<&str, Instructions> {
    tuple((
        multispace0,
        terminated(many1(one_of("LR")), newline),
        newline,
        parse_nodes,
        newline,
    ))(i)
    .map(|(i, (_, directions, _, nodes, _))| {
        (
            i,
            Instructions {
                directions: directions.into_iter().map(|v| LR::from(v)).collect(),
                nodes,
            },
        )
    })
}

pub fn main() {
    let (_, instructions) = all_consuming(parse_instructions)(INPUT).finish().unwrap();

    let mut jump_count = 1;

    let mut it = instructions.nodes.get(&"AAA").unwrap();
    for d in instructions.directions.iter().cycle() {
        let key = match d {
            LR::Left => it.0,
            LR::Right => it.1,
        };

        if key == "ZZZ" {
            break;
        }

        jump_count += 1;

        it = instructions.nodes.get(key).unwrap();
    }

    println!("It took {jump_count} jumps to go from AAA to ZZZ");
}
