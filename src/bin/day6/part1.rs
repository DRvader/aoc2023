use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, newline, space1},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{terminated, tuple},
    Finish, IResult,
};

const INPUT: &'static str = include_str!("input.txt");

/*
const INPUT: &'static str = r#"
Time:      7  15   30
Distance:  9  40  200
"#;
*/

fn parse_race(i: &str) -> IResult<&str, Vec<(f64, f64)>> {
    let (i, times) = tuple((
        multispace0,
        tag("Time:"),
        space1,
        terminated(separated_list1(space1, digit1), newline),
    ))(i)
    .map(|(i, (_, _, _, times))| (i, times.into_iter().map(|v| v.parse().unwrap())))?;
    let (i, distance) = tuple((
        tag("Distance:"),
        space1,
        terminated(separated_list1(space1, digit1), newline),
    ))(i)
    .map(|(i, (_, _, distance))| (i, distance.into_iter().map(|v| v.parse().unwrap())))?;

    Ok((i, times.into_iter().zip(distance.into_iter()).collect()))
}

// Have to determine the number of ways that I could beat the race record.
// The easiest way to do this is actaully do the math.
//
// The longer I hold down the button, the more the velocity increases. So my function is
// d = x * (t - x)
// where
//  d is the final distance
//  x is the time the button is held down
//  t is the total time
//
// The objective is now to find the point when this is greater than or equal to a certain number
// so I add D, which is the distance to beat and set an equality where my formula must be greater
// than it.
// D < x * (t - x)
// 0 < tx - x^2 - D
// -x^2 + tx - D > 0
// t and d are constants so I can just plug this into the quadtratic formula
// x = (-t +- sqrt(t ^ 2 - 4D)) / -2
// the distance between two points will give me the start and stop where the boat wins the race
pub fn main() {
    let (_, races) = all_consuming(parse_race)(INPUT).finish().unwrap();

    let mut sum = 1;
    for (time, distance) in races {
        let rhs = (time.powi(2) - (4.0 * distance)).sqrt();

        let a = -time + rhs;
        let b = -time - rhs;

        let a = a / -2.0;
        let b = b / -2.0;

        // It's a bit of a hack, but the values I get are exclusive. So perfect values need to be
        // shifted.
        let a = (a + 0.0001).ceil();
        let b = (b - 0.0001).floor();

        sum *= (b - a) as usize + 1;
    }

    println!("The total is {sum}");
}
