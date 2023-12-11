use std::collections::{HashMap, HashSet};

const INPUT: &'static str = include_str!("input.txt");

/*
const INPUT: &'static str = r#"
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
"#;
*/

/*
const INPUT: &'static str = r#"
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
"#;
*/

/*
const INPUT: &'static str = r#"
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"#;
*/

struct Tile {
    neighbours: Vec<usize>,
}

enum TileType {
    Vertical,
    Horizontal,
    NEbend,
    NWbend,
    SWbend,
    SEbend,
    Empty,
    FourWay,
}

impl std::fmt::Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            TileType::Vertical => "|",
            TileType::Horizontal => "-",
            TileType::NEbend => "L",
            TileType::NWbend => "J",
            TileType::SWbend => "7",
            TileType::SEbend => "F",
            TileType::Empty => ".",
            TileType::FourWay => "S",
        })
    }
}

pub fn main() {
    let mut tiles = Vec::new();

    let mut start = None;

    let mut line_length = None;
    let mut line_count = 0;
    for (line_idx, line) in INPUT.lines().enumerate() {
        if line.is_empty() {
            continue;
        }

        let new_count = line.chars().count();
        if let Some(line_length) = &line_length {
            assert_eq!(new_count, *line_length);
        }
        line_length = Some(new_count);

        for (col_idx, col) in line.chars().enumerate() {
            match col {
                '|' => {
                    tiles.push(TileType::Vertical);
                }
                '-' => {
                    tiles.push(TileType::Horizontal);
                }
                'L' => {
                    tiles.push(TileType::NEbend);
                }
                'J' => {
                    tiles.push(TileType::NWbend);
                }
                '7' => {
                    tiles.push(TileType::SWbend);
                }
                'F' => {
                    tiles.push(TileType::SEbend);
                }
                '.' => {
                    tiles.push(TileType::Empty);
                }
                'S' => {
                    assert!(start.is_none());
                    start = Some((line_idx, col_idx));
                    tiles.push(TileType::FourWay);
                }
                _ => unimplemented!(""),
            }
        }

        line_count += 1;
    }

    let mut graph = Vec::new();
    for _ in 0..tiles.len() {
        graph.push(Tile {
            neighbours: Vec::new(),
        });
    }

    let line_length = line_length.unwrap();
    for line in 0..line_count {
        for col in 0..line_length {
            match tiles[(line * line_length) + col] {
                TileType::Vertical => {
                    if line > 0 {
                        graph[(line * line_length) + col]
                            .neighbours
                            .push(((line - 1) * line_length) + col);
                        graph[(line * line_length) + col]
                            .neighbours
                            .push(((line + 1) * line_length) + col);
                    }
                }
                TileType::Horizontal => {
                    graph[(line * line_length) + col]
                        .neighbours
                        .push((line * line_length) + col + 1);
                    graph[(line * line_length) + col]
                        .neighbours
                        .push((line * line_length) + col - 1);
                }
                TileType::NEbend => {
                    if line > 0 {
                        graph[(line * line_length) + col]
                            .neighbours
                            .push(((line - 1) * line_length) + col);
                        graph[(line * line_length) + col]
                            .neighbours
                            .push((line * line_length) + col + 1);
                    }
                }
                TileType::NWbend => {
                    if line > 0 {
                        graph[(line * line_length) + col]
                            .neighbours
                            .push(((line - 1) * line_length) + col);
                        graph[(line * line_length) + col]
                            .neighbours
                            .push((line * line_length) + col - 1);
                    }
                }
                TileType::SWbend => {
                    graph[(line * line_length) + col]
                        .neighbours
                        .push(((line + 1) * line_length) + col);
                    graph[(line * line_length) + col]
                        .neighbours
                        .push((line * line_length) + col - 1);
                }
                TileType::SEbend => {
                    graph[(line * line_length) + col]
                        .neighbours
                        .push(((line + 1) * line_length) + col);
                    graph[(line * line_length) + col]
                        .neighbours
                        .push((line * line_length) + col + 1);
                }
                TileType::Empty => {}
                TileType::FourWay => {
                    graph[(line * line_length) + col]
                        .neighbours
                        .push((line * line_length) + col + 1);
                    if col > 0 {
                        graph[(line * line_length) + col]
                            .neighbours
                            .push((line * line_length) + col - 1);
                    }
                    if line > 0 {
                        graph[(line * line_length) + col]
                            .neighbours
                            .push(((line - 1) * line_length) + col);
                    }
                    graph[(line * line_length) + col]
                        .neighbours
                        .push(((line + 1) * line_length) + col);
                }
            }
        }
    }

    let mut counts = vec![vec![]; graph.len()];

    let mut worklist: Vec<(usize, usize, HashSet<usize>)> = vec![(
        start.unwrap().0 * line_length + start.unwrap().1,
        0,
        HashSet::new(),
    )];

    let mut visited_set = vec![(0, HashSet::new()); graph.len()];

    while let Some((idx, count, mut visited)) = worklist.pop() {
        if !visited.insert(idx) {
            continue;
        }

        if visited_set[idx].0 < count {
            visited_set[idx] = (count, visited.clone());
        }

        for n in graph[idx].neighbours.iter() {
            counts[idx].push(count);
            worklist.push((*n, count + 1, visited.clone()));
        }
    }

    let start_idx = start.unwrap().0 * line_length + start.unwrap().1;
    let mut start_set = &visited_set[start_idx];

    let down = &visited_set[(start.unwrap().0 + 1) * line_length + start.unwrap().1];
    if down.0 > start_set.0 {
        start_set = down;
    }

    if start.unwrap().0 > 0 {
        let up = &visited_set[(start.unwrap().0 - 1) * line_length + start.unwrap().1];
        if up.0 > start_set.0 {
            start_set = up;
        }
    }

    if start.unwrap().1 > 0 {
        let left = &visited_set[start.unwrap().0 * line_length + start.unwrap().1 - 1];
        if left.0 > start_set.0 {
            start_set = left;
        }
    }

    let right = &visited_set[start.unwrap().0 * line_length + start.unwrap().1 + 1];
    if right.0 > start_set.0 {
        start_set = right;
    }

    let mut containing = HashSet::new();
    let large_loop = start_set.1.clone();
    for line in 0..line_count {
        for col in 0..line_length {
            let idx = line * line_length + col;

            if large_loop.contains(&idx) {
                continue;
            }

            let mut cross_count = 0;
            for cast_col in col..line_length {
                let cast_idx = line * line_length + cast_col;

                if !large_loop.contains(&cast_idx) {
                    continue;
                }

                let direction_change = match tiles[cast_idx] {
                    TileType::Vertical => true,
                    TileType::Horizontal => false,
                    TileType::NEbend => false,
                    TileType::NWbend => false,
                    TileType::SWbend => true,
                    TileType::SEbend => true,
                    TileType::Empty => false,
                    // Just a guess...
                    TileType::FourWay => false,
                };

                if direction_change {
                    cross_count += 1;
                }
            }

            if cross_count % 2 != 0 {
                containing.insert(idx);
            }
        }
    }

    for line in 0..line_count {
        for col in 0..line_length {
            print!("{}", tiles[line * line_length + col]);
        }
        println!("");
    }

    println!("");

    for line in 0..line_count {
        for col in 0..line_length {
            let idx = line * line_length + col;
            if large_loop.contains(&idx) {
                print!("*");
            } else if containing.contains(&idx) {
                print!("I");
            } else {
                print!("O");
            }
        }
        println!("");
    }

    println!("The number of contained tiles is {}", containing.len());
}
