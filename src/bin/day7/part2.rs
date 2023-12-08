use nom::{
    bytes::complete::take_until,
    character::complete::{digit1, space1},
    combinator::all_consuming,
    sequence::tuple,
    Finish, IResult,
};

const INPUT: &'static str = include_str!("input.txt");

/*
const INPUT: &'static str = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;
*/

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Card {
    Ace,
    King,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl Card {
    pub fn to_usize(&self) -> usize {
        match self {
            Card::Ace => 0,
            Card::King => 1,
            Card::Queen => 2,
            Card::Joker => 3,
            Card::Ten => 4,
            Card::Nine => 5,
            Card::Eight => 6,
            Card::Seven => 7,
            Card::Six => 8,
            Card::Five => 9,
            Card::Four => 10,
            Card::Three => 11,
            Card::Two => 12,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Card::Ace => Card::King,
            Card::King => Card::Queen,
            Card::Queen => Card::Ten,
            Card::Ten => Card::Nine,
            Card::Nine => Card::Eight,
            Card::Eight => Card::Seven,
            Card::Seven => Card::Six,
            Card::Six => Card::Five,
            Card::Five => Card::Four,
            Card::Four => Card::Three,
            Card::Three => Card::Two,
            Card::Two => Card::Joker,
            Card::Joker => Card::Ace,
        }
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Joker,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn get_hand_type(cards: &[Card]) -> HandType {
    let mut count = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    for v in cards {
        count[v.to_usize()] += 1;
    }

    let mut full_house = [false, false];
    let mut pairs = 0;
    for c in count {
        if c == 3 {
            full_house[1] = true;
        }
        if c == 2 {
            full_house[0] = true;
            pairs += 1;
        }

        if c == 5 {
            return HandType::FiveOfAKind;
        } else if c == 4 {
            return HandType::FourOfAKind;
        }
    }

    if full_house[0] && full_house[1] {
        return HandType::FullHouse;
    } else if full_house[1] {
        return HandType::ThreeOfAKind;
    } else if pairs == 2 {
        return HandType::TwoPair;
    } else if pairs == 1 {
        return HandType::OnePair;
    } else {
        return HandType::HighCard;
    }
}

impl From<&[Card; 5]> for HandType {
    fn from(value: &[Card; 5]) -> Self {
        let mut card_sets = value.clone();
        let mut jokers = Vec::with_capacity(5);
        for (index, card) in card_sets.iter().enumerate() {
            if let Card::Joker = card {
                jokers.push(index);
            }
        }

        if jokers.len() == 0 {
            return get_hand_type(&card_sets);
        }

        for c in &mut card_sets {
            if let Card::Joker = c {
                *c = c.next();
            }
        }

        let mut greatest = HandType::HighCard;
        loop {
            if let Card::Joker = card_sets[jokers[jokers.len() - 1]] {
                break;
            }

            for c in &mut card_sets {
                if let Card::Joker = c {
                    *c = c.next();
                }
            }

            greatest = greatest.min(get_hand_type(&card_sets));

            for j in jokers.iter().cloned() {
                card_sets[j] = card_sets[j].next();
                if let Card::Joker = card_sets[j] {
                } else {
                    break;
                }
            }
        }

        greatest
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct Hand {
    text: [Card; 5],
    power: HandType,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.power.cmp(&other.power) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => {
                for (s, o) in self.text.iter().zip(other.text.iter()) {
                    match s.cmp(o) {
                        std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                        std::cmp::Ordering::Equal => {}
                        std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                    }
                }
                std::cmp::Ordering::Equal
            }
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        }
    }
}

fn parse_hand(i: &str) -> IResult<&str, (Hand, usize)> {
    tuple((take_until(" "), space1, digit1))(i).map(|(i, (hand, _, bet))| {
        let mut chars = hand.chars();
        let hand = [
            Card::from(chars.next().unwrap()),
            Card::from(chars.next().unwrap()),
            Card::from(chars.next().unwrap()),
            Card::from(chars.next().unwrap()),
            Card::from(chars.next().unwrap()),
        ];
        let power = HandType::from(&hand);
        (i, (Hand { text: hand, power }, bet.parse().unwrap()))
    })
}

pub fn main() {
    let mut hands = Vec::new();
    for line in INPUT.lines() {
        if line.is_empty() {
            continue;
        }
        let (_, (hand, bet)) = all_consuming(parse_hand)(line).finish().unwrap();
        hands.push((hand, bet));
    }

    let hand_len = hands.len();
    hands.sort_by(|a, b| a.0.cmp(&b.0));

    dbg!(&hands);

    let mut sum = 0;
    for (index, (_hand, bet)) in hands.into_iter().enumerate() {
        sum += bet * (hand_len - index);
    }

    println!("The total winnings are {sum}");
}
