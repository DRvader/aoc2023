const INPUT: &'static str = include_str!("input.txt");

pub fn main() {
    let mut sum = 0;
    for line in INPUT.lines() {
        if !line.is_empty() {
            let mut first = None;
            let mut last = None;
            for char in line.chars() {
                if let Some(d) = char.to_digit(10) {
                    first = Some(d);
                    break;
                }
            }

            for char in line.chars().rev() {
                if let Some(d) = char.to_digit(10) {
                    last = Some(d);
                    break;
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
