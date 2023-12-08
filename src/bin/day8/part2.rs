use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace0, newline, one_of, space0},
    combinator::all_consuming,
    multi::{many1, separated_list1},
    sequence::{terminated, tuple},
    Finish, IResult,
};

const INPUT: &'static str = include_str!("input.txt");

/*
const INPUT: &'static str = r#"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
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

    let mut starting = Vec::with_capacity(instructions.nodes.len());
    let mut ending = Vec::with_capacity(instructions.nodes.len());

    let mut jump_counts = Vec::with_capacity(instructions.nodes.len());
    for (i, _) in &instructions.nodes {
        if i.ends_with("A") {
            starting.push(i);

            let it = &mut &**i;

            let mut jump_count = 1;
            for d in instructions.directions.iter().cycle() {
                let i = instructions.nodes.get(&*it).unwrap();
                let key = match d {
                    LR::Left => &i.0,
                    LR::Right => &i.1,
                };

                if key.ends_with("Z") {
                    ending.push(key);
                    break;
                }

                jump_count += 1usize;

                *it = &key;
            }

            jump_counts.push(jump_count);
        }
    }

    let true_initial_counts = jump_counts.clone();

    /*
    let mut initial_counts = true_initial_counts.clone();
    let mut jump_counts = true_initial_counts.clone();
    let output_jump;
    loop {
        let mut next_jump_counts = Vec::new();
        let mut next_initial_jump_counts = Vec::new();
        for i in 0..(jump_counts.len() / 2) {
            let a = i * 2;
            let b = i * 2 + 1;

            while jump_counts[a] != jump_counts[b] {
                if jump_counts[a] > jump_counts[b] {
                    jump_counts[b] += initial_counts[b];
                } else if jump_counts[b] > jump_counts[a] {
                    jump_counts[a] += initial_counts[a];
                } else {
                    break;
                }
            }

            next_jump_counts.push(jump_counts[a]);
            next_initial_jump_counts.push(jump_counts[a]);
        }

        if jump_counts.len() % 2 != 0 {
            next_jump_counts.push(jump_counts[jump_counts.len() - 1]);
            next_initial_jump_counts.push(initial_counts[jump_counts.len() - 1]);
        }

        if next_jump_counts.len() == 1 {
            output_jump = next_jump_counts[0];
            break;
        }

        jump_counts = next_jump_counts;
        initial_counts = next_initial_jump_counts;
    }
    */

    let mut initial_counts = true_initial_counts.clone();
    let mut jump_counts = true_initial_counts.clone();
    let mut jump_count = jump_counts.len();
    while jump_count > 1 {
        for i in 0..(jump_count / 2) {
            let a = i * 2;
            let b = i * 2 + 1;

            while jump_counts[a] != jump_counts[b] {
                if jump_counts[a] > jump_counts[b] {
                    jump_counts[b] += initial_counts[b];
                } else if jump_counts[b] > jump_counts[a] {
                    jump_counts[a] += initial_counts[a];
                } else {
                    break;
                }
            }

            jump_counts[i] = jump_counts[a];
            initial_counts[i] = jump_counts[a];
        }

        jump_count = if jump_count % 2 != 0 {
            jump_counts[jump_count / 2] = jump_counts[jump_count - 1];
            initial_counts[jump_count / 2] = initial_counts[jump_count - 1];

            jump_count / 2 + 1
        } else {
            jump_count / 2
        };
    }

    let output_jump = jump_counts[0];

    println!("It took {output_jump} jumps to go from {starting:?} to {ending:?}");
}
