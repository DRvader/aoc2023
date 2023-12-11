use std::collections::{HashMap, HashSet};

const INPUT: &'static str = include_str!("input.txt");

/*
const INPUT: &'static str = r#"
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
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

    while let Some((idx, count, visited)) = worklist.pop() {
        if visited.contains(&idx) {
            continue;
        }

        for n in graph[idx].neighbours.iter() {
            counts[idx].push(count);

            let mut visited = visited.clone();
            visited.insert(idx);

            worklist.push((*n, count + 1, visited));
        }
    }

    let mut valid_counts = Vec::new();

    let mut map: HashMap<usize, usize> = HashMap::new();
    for c in counts {
        map.clear();

        for i in c {
            *map.entry(i).or_default() += 1;
        }

        map.retain(|_k, v| *v >= 2);

        valid_counts.push(map.keys().copied().max().unwrap_or(0));
    }

    /*
    for line in 0..line_count {
        for col in 0..line_length {
            print!("{}", tiles[line * line_length + col]);
        }
        println!("");
    }

    println!("");

    for line in 0..line_count {
        for col in 0..line_length {
            if let TileType::Empty = tiles[line * line_length + col] {
                print!("|..|");
            } else {
                print!("|{:02}|", valid_counts[line * line_length + col]);
            }
        }
        println!("");
    }
    */

    let mut max = 0;
    for count in valid_counts {
        max = max.max(count);
    }

    // The values we have are proven loops therefore they will be twice as large as the middle
    // point.
    println!("The maximum distance is {}", (max + 1) / 2);
}
