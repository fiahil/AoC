use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Hand {
    cards: Vec<Card>,
    original_order: Vec<Card>,
}

impl Hand {
    fn new(cards: Vec<Card>) -> Self {
        Self {
            cards: cards.clone().into_iter().sorted().collect(),
            original_order: cards,
        }
    }

    fn score(&self) -> u32 {
        let count_jokers = self.cards.iter().filter(|x| **x == Card::Joker).count();
        let groups = self
            .cards
            .iter()
            .group_by(|x| *x)
            .into_iter()
            .map(|(k, v)| (k, v.count()))
            .collect::<HashMap<_, _>>();

        if groups
            .iter()
            .any(|(c, x)| (*x + count_jokers) == 5 && **c != Card::Joker)
            || count_jokers == 5
        {
            7 // Five of a kind
        } else if groups
            .iter()
            .any(|(c, x)| (*x + count_jokers) == 4 && **c != Card::Joker)
        {
            6 // Four of a kind
        } else if groups.values().any(|x| (*x + count_jokers) == 3)
            && groups.values().any(|x| *x == 2)
            && groups.values().filter(|x| **x >= 2).count() == 2
        {
            5 // Full house
        } else if groups
            .iter()
            .any(|(c, x)| (*x + count_jokers) == 3 && **c != Card::Joker)
        {
            4 // Three of a kind
        } else if groups.values().filter(|x| **x == 2).count() == 2 {
            3 // Two pair
        } else if groups
            .iter()
            .any(|(c, x)| (*x + count_jokers) == 2 && **c != Card::Joker)
        {
            2 // One pair
        } else {
            1 // High card
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_score = self.score();
        let other_score = other.score();

        // println!(
        //     "self = {:?} ({}) | other = {:?} ({})",
        //     self, self_score, other, other_score
        // );

        if self_score != other_score {
            return self_score.cmp(&other_score);
        }

        // println!(
        //     "TIE BREAKER = {:?}",
        //     self.cards
        //         .iter()
        //         .zip(other.cards.iter())
        //         .fold(Ordering::Equal, |acc, (a, b)| {
        //             if acc != Ordering::Equal {
        //                 return acc;
        //             }
        //             println!("a = {:?} | b = {:?} | result = {:?}", a, b, a.cmp(&b));
        //             a.cmp(&b)
        //         })
        // );

        self.original_order
            .iter()
            .zip(other.original_order.iter())
            .fold(Ordering::Equal, |acc, (a, b)| {
                if acc != Ordering::Equal {
                    return acc;
                }
                a.cmp(&b)
            })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Card {
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

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        let v = vec![
            Card::Joker,
            Card::Two,
            Card::Three,
            Card::Four,
            Card::Five,
            Card::Six,
            Card::Seven,
            Card::Eight,
            Card::Nine,
            Card::Ten,
            Card::Queen,
            Card::King,
            Card::Ace,
        ];

        v.iter()
            .position(|x| x == self)
            .unwrap()
            .cmp(&v.iter().position(|x| x == other).unwrap())
    }
}

impl From<char> for Card {
    fn from(s: char) -> Self {
        match s {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            'J' => Card::Joker,
            _ => panic!("Unknown card: {}", s),
        }
    }
}

pub fn part1(input: &String) -> u32 {
    let mut hands: BTreeMap<Hand, u32> = BTreeMap::new();
    for line in input.lines() {
        let (hand, score) = line.split_once(" ").unwrap();
        let cards: Vec<Card> = hand.chars().map(Card::from).collect();
        let score = score.parse::<u32>().unwrap();

        hands.insert(Hand::new(cards), score);
    }

    hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, (_, s))| acc + ((i as u32 + 1) * s))
}

pub fn part2(input: &String) -> u32 {
    let mut hands: BTreeMap<Hand, u32> = BTreeMap::new();
    for line in input.lines() {
        let (hand, score) = line.split_once(" ").unwrap();
        let cards: Vec<Card> = hand.chars().map(Card::from).collect();
        let score = score.parse::<u32>().unwrap();
        let hand = Hand::new(cards);

        println!("score = {} | {} | hand = {:?}", score, hand.score(), hand);

        hands.insert(hand, score);
    }

    hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, (_, s))| acc + ((i as u32 + 1) * s))
}

pub mod test {
    pub fn part1(input: &String) -> u32 {
        let r = super::part1(input);
        assert_eq!(r, 6440);

        r
    }

    pub fn part2(input: &String) -> u32 {
        let r = super::part2(input);
        assert_eq!(r, 5905);

        r
    }
}
