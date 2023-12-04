const INPUT: &'static str = include_str!("input.txt");

/*
const INPUT: &'static str = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;
*/

#[derive(Debug, Clone, Copy)]
pub enum Component {
    Number(usize),
    Gear,
    Empty,
}

macro_rules! get_grid_values {
    ($grid:ident, $x:ident, $y:ident) => {
        [
            if $y > 0 && $x > 0 {
                Some($grid[$y - 1][$x - 1])
            } else {
                None
            },
            if $y > 0 {
                Some($grid[$y - 1][$x])
            } else {
                None
            },
            if $x > 0 {
                Some($grid[$y][$x - 1])
            } else {
                None
            },
            if $y + 1 < $grid.len() && $x + 1 < $grid[$y].len() {
                Some($grid[$y + 1][$x + 1])
            } else {
                None
            },
            if $y + 1 < $grid.len() {
                Some($grid[$y + 1][$x])
            } else {
                None
            },
            if $x + 1 < $grid[$y].len() {
                Some($grid[$y][$x + 1])
            } else {
                None
            },
            if $x + 1 < $grid[$y].len() && $y > 0 {
                Some($grid[$y - 1][$x + 1])
            } else {
                None
            },
            if $x > 0 && $y + 1 < $grid.len() {
                Some($grid[$y + 1][$x - 1])
            } else {
                None
            },
        ]
    };
}

pub fn main() {
    let mut grid = Vec::new();
    let mut numbers = Vec::new();

    // The general design for part 1 will be to, setup a grid that'll let us do a convolution
    // style scan to chcek if a number is next to a symbol (anything that isn't '.').
    // THen actually perform the scan.

    for line in INPUT.lines() {
        let mut running_number = 0;
        let mut row = Vec::new();

        for char in line.chars() {
            if let Some(d) = char.to_digit(10) {
                running_number *= 10;
                running_number += d;
                row.push(Component::Number(numbers.len()));
            } else {
                if running_number > 0 {
                    numbers.push(running_number);
                }
                running_number = 0;

                let comp = if char == '*' {
                    Component::Gear
                } else {
                    Component::Empty
                };

                row.push(comp);
            }
        }

        if running_number > 0 {
            numbers.push(running_number);
        }

        grid.push(row);
    }

    let mut sum = 0;

    for y in 0..grid.len() {
        'outer: for x in 0..grid[y].len() {
            if let Component::Gear = grid[y][x] {
                let values = get_grid_values!(grid, x, y);

                let mut seen_1 = None;
                let mut seen_2 = None;
                for value in values {
                    if let Some(value) = value {
                        if let Component::Number(num) = value {
                            if seen_1.is_none() {
                                seen_1 = Some(num);
                            } else if seen_2.is_none() && seen_1 != Some(num) {
                                seen_2 = Some(num);
                            } else if seen_2.is_some() && Some(num) != seen_2 && Some(num) != seen_1 {
                                continue 'outer;
                            }
                        }
                    }
                }

                if let (Some(seen_1), Some(seen_2)) = (seen_1, seen_2) {
                    sum += numbers[seen_1] * numbers[seen_2];
                }
            }
        }
    }

    println!("The engine part number is {sum}");
}
