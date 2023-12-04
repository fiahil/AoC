#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl From<&str> for Card {
    fn from(s: &str) -> Self {
        let (left, right) = s.split_once(":").unwrap().1.split_once("|").unwrap();
        let winning_numbers = left
            .split_whitespace()
            .map(|n| n.trim().parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let numbers = right
            .split_whitespace()
            .map(|n| n.trim().parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        Self {
            winning_numbers,
            numbers,
        }
    }
}

pub fn part1(input: &String) -> u32 {
    let cards = input.lines().map(Card::from).collect::<Vec<Card>>();

    let mut scores = Vec::new();
    for card in cards {
        let score = card
            .numbers
            .iter()
            .filter(|n| card.winning_numbers.contains(n))
            .fold(0, |acc, _| if acc == 0 { acc + 1 } else { acc * 2 });
        scores.push(score);
    }

    scores.iter().sum()
}

pub fn part2(input: &String) -> u32 {
    let cards = input.lines().map(Card::from).collect::<Vec<Card>>();

    fn aux(cards: &Vec<Card>, score: u32, index: usize, copy: &str) -> u32 {
        let card = &cards[index];
        let count = card
            .numbers
            .iter()
            .filter(|n| card.winning_numbers.contains(n))
            .count();

        println!("Card {} ({}) : {count} ({score})", index + 1, copy);

        let mut score = score;
        for i in 1..(count + 1) {
            if index + i < cards.len() {
                score = aux(
                    cards,
                    score + 1,
                    index + i,
                    &format!("copy of {}", index + 1),
                );
            }
        }

        score
    }

    let mut score = 0;
    for i in 0..cards.len() {
        score = aux(&cards, score, i, "originale");
        score += 1;
    }

    println!("Score: {}", score);

    score
}

pub mod test {
    pub fn part1(input: &String) -> u32 {
        let r = super::part1(input);
        assert_eq!(r, 13);

        r
    }

    pub fn part2(input: &String) -> u32 {
        let r = super::part2(input);
        assert_eq!(r, 30);

        r
    }
}
