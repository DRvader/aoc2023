use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{digit1, multispace0, multispace1, newline, space0, space1},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::tuple,
    Finish, IResult,
};

const INPUT: &'static str = include_str!("input.txt");

/*
const INPUT: &'static str = r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;
*/

#[derive(Debug)]
pub struct Range {
    dest_start: usize,
    source_start: usize,
    len: usize,
}

fn parse_range(i: &str) -> IResult<&str, Range> {
    tuple((digit1, space1, digit1, space1, digit1, space0))(i).map(
        |(i, (dest_start, _, source_start, _, len, _))| {
            (
                i,
                Range {
                    dest_start: dest_start.parse().unwrap(),
                    source_start: source_start.parse().unwrap(),
                    len: len.parse().unwrap(),
                },
            )
        },
    )
}

fn parse_map(i: &str) -> IResult<&str, (&str, &str, Vec<Range>)> {
    tuple((
        take_until("-"),
        tag("-to-"),
        take_until(" map:"),
        tag(" map:"),
        newline,
        separated_list1(newline, parse_range),
        newline,
    ))(i)
    .map(|(i, (from, _, to, _, _, ranges, _))| (i, (from, to, ranges)))
}

fn parse_seed_conversion(i: &str) -> IResult<&str, usize> {
    let (i, (_, _, seeds, _)) = tuple((
        multispace0,
        tag("seeds: "),
        separated_list1(multispace1, digit1),
        multispace1,
    ))(i)?;
    let seeds = seeds
        .into_iter()
        .map(|v| v.parse().unwrap())
        .collect::<Vec<usize>>();

    let (i, mut maps) = separated_list1(newline, parse_map)(i)?;

    // Looking at the input I know that going over the maps in order will perfectly map from seeds
    // to location.
    // So if I iterate in reverse I will go from location to seeds.

    for (_, _, map) in maps.iter_mut() {
        map.sort_by_key(|v| v.dest_start);
    }

    let mut lowest_location = usize::MAX;

    let mut worklist = Vec::new();
    for loc in &seeds {
        worklist.push((0, *loc));
    }

    while let Some((target_list, current_value)) = worklist.pop() {
        if target_list < maps.len() {
            let mut pushed = false;
            for value in &maps[target_list].2 {
                if (current_value >= value.source_start)
                    && (current_value <= (value.source_start + value.len))
                {
                    pushed = true;
                    worklist.push((
                        target_list + 1,
                        value.dest_start + (current_value - value.source_start),
                    ));
                }
            }

            if !pushed {
                worklist.push((target_list + 1, current_value));
            }
        } else if lowest_location > current_value {
            lowest_location = current_value;
        }
    }

    Ok((i, lowest_location))
}

pub fn main() {
    let (_, lowest_location) = all_consuming(parse_seed_conversion)(INPUT)
        .finish()
        .unwrap();
    println!("The lowest seed value is {lowest_location}");
}
