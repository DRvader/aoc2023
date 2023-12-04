const INPUT: &'static str = include_str!("input.txt");

pub enum Component {
    Number(usize),
    Symbol,
    Empty,
}

impl Component {
    fn is_symbol(&self) -> bool {
        match self {
            Component::Number(_) => false,
            Component::Symbol => true,
            Component::Empty => false,
        }
    }
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

                let comp = if char == '.' {
                    Component::Empty
                } else {
                    Component::Symbol
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

    let mut already_added = vec![false; numbers.len()];
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if let Component::Number(num) = grid[y][x] {
                if already_added[num] {
                    continue;
                }

                if (y > 0 && x > 0 && grid[y - 1][x - 1].is_symbol())
                    || (y > 0 && grid[y - 1][x].is_symbol())
                    || (x > 0 && grid[y][x - 1].is_symbol())
                    || (y + 1 < grid.len()
                        && x + 1 < grid[y].len()
                        && grid[y + 1][x + 1].is_symbol())
                    || (y + 1 < grid.len() && grid[y + 1][x].is_symbol())
                    || (x + 1 < grid[y].len() && grid[y][x + 1].is_symbol())
                    || (x + 1 < grid[y].len() && y > 0 && grid[y - 1][x + 1].is_symbol())
                    || (x > 0 && y + 1 < grid.len() && grid[y + 1][x - 1].is_symbol())
                {
                    sum += numbers[num];
                    already_added[num] = true;
                }
            }
        }
    }

    println!("The engine part number is {sum}");
}
