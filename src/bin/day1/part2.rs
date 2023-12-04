use std::borrow::Cow;

const INPUT: &'static str = include_str!("input.txt");

/*
const INPUT: &'static str = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;
*/

pub fn build_patterns() -> Vec<(usize, Cow<'static, [char]>, usize)> {
    vec![
        (0, Cow::Owned("one".chars().collect()), 1),
        (0, Cow::Owned("two".chars().collect()), 2),
        (0, Cow::Owned("three".chars().collect()), 3),
        (0, Cow::Owned("four".chars().collect()), 4),
        (0, Cow::Owned("five".chars().collect()), 5),
        (0, Cow::Owned("six".chars().collect()), 6),
        (0, Cow::Owned("seven".chars().collect()), 7),
        (0, Cow::Owned("eight".chars().collect()), 8),
        (0, Cow::Owned("nine".chars().collect()), 9),
        (0, Cow::Owned(vec!['0']), 0),
        (0, Cow::Owned(vec!['1']), 1),
        (0, Cow::Owned(vec!['2']), 2),
        (0, Cow::Owned(vec!['3']), 3),
        (0, Cow::Owned(vec!['4']), 4),
        (0, Cow::Owned(vec!['5']), 5),
        (0, Cow::Owned(vec!['6']), 6),
        (0, Cow::Owned(vec!['7']), 7),
        (0, Cow::Owned(vec!['8']), 8),
        (0, Cow::Owned(vec!['9']), 9),
    ]
}

pub fn main() {
    let mut patterns = build_patterns();

    let mut sum = 0;
    for line in INPUT.lines() {
        if !line.is_empty() {
            let mut first = None;
            let mut last = None;

            // Reset patterns for new digit
            for p in &mut patterns {
                p.0 = 0;
            }

            'outer: for char in line.chars() {
                for p in &mut patterns {
                    if char == p.1[p.0] || char == p.1[0] {
                        if char == p.1[0] && char != p.1[p.0] {
                            p.0 = 0;
                        }

                        p.0 += 1;
                        if p.0 == p.1.len() {
                            first = Some(p.2);
                            break 'outer;
                        }
                    }
                }
            }

            // This time we are running in reverse so we parse the pattern backwards.
            // Reset patterns for new REVERSED digit
            for p in &mut patterns {
                p.0 = p.1.len();
            }

            'outer: for char in line.chars().rev() {
                for p in &mut patterns {
                    // Check the current digit in the pattern (we use -1 here so that we can
                    // compare against 0. We do this offset beacuse the type is usize, and
                    // therefore will overflow if we try to subtract past 0.
                    if char == p.1[p.0 - 1] || char == p.1[p.1.len() - 1] {
                        // If the first check was wrong then we want to also cehck against the
                        // start of the pattern in order to handle the case where if we reset the
                        // iterator then the pattern matches.
                        //
                        // It's very important to make sure that we check that we haven't matched
                        // on the current character index, otherwise repeated characters in the
                        // pattern will cause problems.
                        if char == p.1[p.1.len() - 1] && char != p.1[p.0 - 1] {
                            p.0 = p.1.len();
                        }

                        p.0 -= 1;
                        // If the iterator has found a matching pattern then we set our digit.
                        if p.0 == 0 {
                            last = Some(p.2);
                            // We don't have to reset anything because we immediatly exit the outer
                            // loop and will reset before starting to search again.
                            break 'outer;
                        }
                    }
                }
            }

            if let (Some(first), Some(last)) = (first, last) {
                let num = first * 10 + last;
                sum += num;
            }
        }
    }

    println!("The sum of the input is: {sum}");
}
