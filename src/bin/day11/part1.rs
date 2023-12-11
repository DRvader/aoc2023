const INPUT: &'static str = include_str!("input.txt");

/*
const INPUT: &'static str = r#"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#;
*/

pub fn main() {
    let mut galaxy_data = Vec::new();

    let mut line_count = 0;
    let mut line_length = None;
    for line in INPUT.lines() {
        if line.is_empty() {
            continue;
        }

        let mut char_length = 0;
        for char in line.chars() {
            let empty = match char {
                '.' => true,
                '#' => false,
                _ => unimplemented!(),
            };
            galaxy_data.push(empty);

            char_length += 1;
        }

        if let Some(line_length) = line_length {
            assert_eq!(line_length, char_length);
        } else {
            line_length = Some(char_length);
        }

        line_count += 1;
    }

    let line_length = line_length.unwrap();

    let mut positions = Vec::new();
    for l in 0..line_count {
        for c in 0..line_length {
            if !galaxy_data[l * line_length + c] {
                positions.push((l as isize, c as isize));
            }
        }
    }

    for l in (0..line_count).rev() {
        let mut empty_l = true;
        for c in 0..line_length {
            if !galaxy_data[l * line_length + c] {
                empty_l = false;
                break;
            }
        }

        if empty_l {
            for (pl, _) in positions.iter_mut() {
                if *pl as usize >= l {
                    *pl += 1;
                }
            }
        }
    }

    for c in (0..line_length).rev() {
        let mut empty_c = true;
        for l in 0..line_count {
            if !galaxy_data[l * line_length + c] {
                empty_c = false;
                break;
            }
        }

        if empty_c {
            for (_, pc) in positions.iter_mut() {
                if *pc as usize >= c {
                    *pc += 1;
                }
            }
        }
    }

    let mut sum = 0;
    for (aidx, a) in positions.iter().enumerate() {
        for (bidx, b) in positions.iter().enumerate().skip(aidx + 1) {
            if aidx == bidx {
                continue;
            }

            let mx = b.0.abs_diff(a.0);
            let my = b.1.abs_diff(a.1);

            let distance = mx + my;
            sum += distance;
        }
    }

    println!("The sum of the shortest distances is {sum}");
}
