use nom::{
    character::complete::{char, digit1, multispace0, space1},
    combinator::{all_consuming, opt, recognize},
    multi::separated_list1,
    sequence::pair,
    Finish, IResult,
};

const INPUT: &'static str = include_str!("input.txt");

/*
const INPUT: &'static str = r#"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#;
*/

fn parse_values(i: &str) -> IResult<&str, Vec<isize>> {
    let (i, _) = multispace0(i)?;
    separated_list1(space1, recognize(pair(opt(char('-')), digit1)))(i)
        .map(|(i, v)| (i, v.into_iter().map(|v| v.parse().unwrap()).collect()))
}

pub fn main() {
    let mut sum = 0;

    let mut differences = Vec::new();
    for line in INPUT.lines() {
        if line.is_empty() {
            continue;
        }

        let (_, values) = all_consuming(parse_values)(line).finish().unwrap();

        if differences.len() == 0 {
            differences.push(Vec::new());
        }

        differences[0] = values;

        let mut index = 1;
        loop {
            while index >= differences.len() {
                differences.push(Vec::new());
            }

            differences[index].clear();

            let (old, new) = differences.split_at_mut(index);

            for v in old[old.len() - 1].windows(2) {
                let diff = v[1] - v[0];

                new[0].push(diff);
            }

            if differences[index].iter().all(|v| *v == 0) {
                break;
            }

            index += 1;
        }

        let pattern_count = index;

        for index in (0..(pattern_count - 1)).rev() {
            let new_value = differences[index][differences[index].len() - 1]
                + differences[index + 1][differences[index + 1].len() - 1];
            differences[index].push(new_value);
        }

        sum += differences[0][differences[0].len() - 1];
    }

    println!("The sum of the extrapolated values is {sum}");
}
