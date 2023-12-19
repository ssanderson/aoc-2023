mod hand;

use hand::{BidHand, Card, Hand};

use utils::Part1;

struct Problem7;

impl Part1 for Problem7 {
    const N: u8 = 7;
    type Input = Input;

    fn parse(data: &str) -> anyhow::Result<Self::Input> {
        utils::parse::finalize(parser::parse_input(data.trim()))
    }

    fn run1(input: Self::Input) -> anyhow::Result<String> {
        let mut hands = input.hands;
        hands.sort_by_key(|h| h.hand);

        let total: usize = hands
            .into_iter()
            .enumerate()
            .map(|(i, h)| (i + 1) * h.bid as usize)
            .sum();

        Ok(total.to_string())
    }
}

struct Input {
    hands: Vec<BidHand>,
}

mod parser {
    use nom::bytes::complete::tag;
    use nom::character::complete::one_of;
    use nom::combinator::all_consuming;
    use nom::multi::separated_list1;
    use nom::sequence::tuple;
    use nom::Parser;
    use nom::{sequence::separated_pair, IResult};

    use super::{BidHand, Card, Hand, Input};

    fn card(input: &str) -> IResult<&str, Card> {
        one_of("AKQJ23456789T")
            .map(|c| match c {
                'A' => Card::Ace,
                'K' => Card::King,
                'Q' => Card::Queen,
                'J' => Card::Jack,
                '2' => Card::Two,
                '3' => Card::Three,
                '4' => Card::Four,
                '5' => Card::Five,
                '6' => Card::Six,
                '7' => Card::Seven,
                '8' => Card::Eight,
                '9' => Card::Nine,
                'T' => Card::Ten,
                _ => panic!("bad card: {c}"),
            })
            .parse(input)
    }

    pub fn parse_input(input: &str) -> IResult<&str, Input> {
        let cards = tuple((card, card, card, card, card));
        let bid = utils::parse::num::<u32>;
        let hand = separated_pair(cards, tag(" "), bid).map(|((a, b, c, d, e), n)| BidHand {
            hand: Hand::from([a, b, c, d, e]),
            bid: n,
        });
        let parser = separated_list1(tag("\n"), hand);

        all_consuming(parser)
            .map(|hands| Input { hands })
            .parse(input)
    }
}

fn main() -> anyhow::Result<()> {
    utils::run_part1::<Problem7>()?;
    Ok(())
}
