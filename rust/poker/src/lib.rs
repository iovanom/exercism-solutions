/// Given a list of poker hands, return a list of those hands which win.
///
/// Suits:
/// clubs (♣), diamonds (♦), hearts (♥) and spades (♠)
///
/// Rank:
/// A, K, Q, J, 10, 9, 8, 7, 6, 5, 4, 3 and 2
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
use std::fmt;

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut result = vec![];
    for hand in hands {
        let poker_hand = PokerHand::new(hand).ok()?;
        result.push(poker_hand);
    }
    result.sort_by(|a, b| b.hand_value().cmp(&a.hand_value()));
    for r in result.clone() {
        println!("{:?} - {}", r, r.hand_value());
    }
    let max_score = result[0].hand_value();
    Some(
        result
            .iter()
            .filter(|h| h.hand_value() == max_score)
            .map(|h| h.hand)
            .collect(),
    )
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum Suit {
    C = 1,
    D = 2,
    H = 3,
    S = 4,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone)]
struct Card(u8, Suit);

impl fmt::Debug for Card {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let Card(rank, suit) = self;
        let suit = match suit {
            Suit::C => '♣',
            Suit::D => '♦',
            Suit::H => '♥',
            Suit::S => '♠',
        };
        let rank = match rank {
            14 => "A".to_string(),
            13 => "K".to_string(),
            12 => "Q".to_string(),
            11 => "J".to_string(),
            _ => rank.to_string(),
        };
        formatter.write_fmt(format_args!("{}{}", rank, suit))
    }
}

#[derive(Debug, Clone)]
struct PokerHand<'a> {
    cards: Vec<Card>,
    hand: &'a str,
    value: Option<u32>,
}

impl<'a> PokerHand<'a> {
    const CARDS_COUNT: u8 = 5;

    // Rating consts
    const STRAIGHT_FLASH: u32 = 8000000;
    const FOUR_OF_A_KIND: u32 = 7000000;
    const FULL_HOUSE: u32 = 6000000;
    const FLUSH: u32 = 5000000;
    const STRAIGHT: u32 = 4000000;
    const SET: u32 = 3000000;
    const TWO_PAIRS: u32 = 2000000;
    const ONE_PAIR: u32 = 1000000;

    fn new(hand: &'a str) -> Result<PokerHand, String> {
        let mut cards: Vec<Card> = Vec::with_capacity(5);
        for (index, str_card) in hand.split(' ').enumerate() {
            if index >= Self::CARDS_COUNT as usize {
                return Err(format!("to many cards"));
            }
            let rank: u8 = match &str_card[0..str_card.len() - 1] {
                "A" => 14,
                "K" => 13,
                "Q" => 12,
                "J" => 11,
                r => r
                    .parse()
                    .map_err(|_| format!("error on parse rank {}", str_card))?,
            };
            let suit = match str_card.chars().nth(str_card.len() - 1) {
                Some('C') => Suit::C,
                Some('D') => Suit::D,
                Some('H') => Suit::H,
                Some('S') => Suit::S,
                _ => return Err(format!("wrong suit format for {}", str_card)),
            };
            cards.push(Card(rank, suit));
        }
        cards.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.partial_cmp(&b.1).unwrap()));
        let mut hand_obj = PokerHand {
            cards,
            hand,
            value: None,
        };
        hand_obj.calc_value();
        Ok(hand_obj)
    }

    fn hand_value(&self) -> u32 {
        self.value.unwrap()
    }

    fn calc_value(&mut self) {
        let value = if self.is_straight_flush() {
            self.value_traight_flush()
        } else if self.is_4s() {
            self.value_four_kind()
        } else if self.is_full_house() {
            self.value_full_house()
        } else if self.is_flush() {
            self.value_flush()
        } else if self.is_straight() {
            if self.is_straight_start_with_a() {
                self.value_straight_start_with_a()
            } else {
                self.value_straight()
            }
        } else if self.is_3s() {
            self.value_set()
        } else if self.is_two_pairs() {
            self.value_two_pairs()
        } else if self.is_pair() {
            self.value_pair()
        } else {
            self.value_hight_card()
        };
        self.value = Some(value);
    }

    fn is_flush(&self) -> bool {
        self.cards
            .windows(2)
            .map(|win| {
                if let [Card(_, a), Card(_, b)] = win {
                    a == b
                } else {
                    false
                }
            })
            .all(|r| r)
    }

    fn is_straight(&self) -> bool {
        self.cards[0].0 + PokerHand::CARDS_COUNT - 1 == self.cards[self.cards.len() - 1].0
            && !self.is_pair()
            || self.is_straight_start_with_a()
    }

    fn is_straight_start_with_a(&self) -> bool {
        self.cards[self.cards.len() - 1].0 == 14
            && self.cards[0].0 + PokerHand::CARDS_COUNT - 2 == self.cards[self.cards.len() - 2].0
            && !self.is_pair()
    }

    fn is_straight_flush(&self) -> bool {
        self.is_flush() && self.is_straight()
    }

    fn is_4s(&self) -> bool {
        self.cards.windows(4).any(|cards| {
            cards[0].0 == cards[1].0 && cards[1].0 == cards[2].0 && cards[2].0 == cards[3].0
        })
    }

    fn is_3s(&self) -> bool {
        self.cards.windows(3).any(|win| {
            if let [Card(a, _), Card(b, _), Card(c, _)] = win {
                a == b && b == c
            } else {
                false
            }
        })
    }

    fn is_full_house(&self) -> bool {
        // check variant 3 3 3 2 2 || 2 2 3 3 3
        self.cards[0].0 == self.cards[2].0 && self.cards[3].0 == self.cards[4].0
            || self.cards[0].0 == self.cards[1].0 && self.cards[2].0 == self.cards[4].0
    }

    fn is_two_pairs(&self) -> bool {
        self.cards
            .windows(2)
            .map(|win| {
                if let [Card(a, _), Card(b, _)] = win {
                    a == b
                } else {
                    false
                }
            })
            .filter(|&r| r)
            .count()
            == 2
    }

    fn is_pair(&self) -> bool {
        self.cards
            .windows(2)
            .map(|win| {
                if let [Card(a, _), Card(b, _)] = win {
                    a == b
                } else {
                    false
                }
            })
            .any(|r| r)
    }

    fn value_traight_flush(&self) -> u32 {
        PokerHand::STRAIGHT_FLASH + self.value_hight_card()
    }

    fn value_flush(&self) -> u32 {
        PokerHand::FLUSH + self.value_hight_card()
    }

    fn value_straight(&self) -> u32 {
        PokerHand::STRAIGHT + self.value_hight_card()
    }

    fn value_straight_start_with_a(&self) -> u32 {
        let v: u32 = self.cards[..self.cards.len() - 1]
            .iter()
            .enumerate()
            .map(|(i, &Card(rank, _))| 14_u32.pow(i as u32) * (rank as u32))
            .sum();
        PokerHand::STRAIGHT + v
    }

    fn value_set(&self) -> u32 {
        let set_rank = self.cards[2].0;
        let c: u32 = self
            .cards
            .iter()
            .filter(|Card(rank, _)| *rank != set_rank)
            .enumerate()
            .map(|(i, Card(rank, _))| 14_u32.pow(i as u32) * (*rank as u32))
            .sum();
        PokerHand::SET + c + (set_rank as u32) * 14_u32.pow(3)
    }

    fn get_ranks(&self) -> (u32, u32, u32, u32, u32) {
        let v = self
            .cards
            .iter()
            .map(|&Card(rank, _)| rank as u32)
            .collect::<Vec<_>>();

        (v[0], v[1], v[2], v[3], v[4])
    }

    /// Get value for two pairs
    ///     value = TWO_PAIRS + 14*14*HighPairCard + 14*LowPairCard + UnmatchedCard
    ///
    fn value_two_pairs(&self) -> u32 {
        let (a, b, c, d, e) = self.get_ranks();
        let calc = |r1, r2, r3| 14_u32.pow(2) * r1 + 14 * r2 + r3;
        let val = if a == b && c == d {
            calc(c, a, e)
        } else if a == b && d == e {
            calc(d, a, c)
        } else {
            calc(d, b, a)
        };
        PokerHand::TWO_PAIRS + val
    }

    fn value_pair(&self) -> u32 {
        let (a, b, c, d, e) = self.get_ranks();
        let calc = |r1, r2, r3, r4| 14_u32.pow(3) * r1 + 14_u32.pow(2) * r2 + 14 * r3 + r4;
        let val = if a == b {
            calc(a, c, d, e)
        } else if b == c {
            calc(b, a, d, e)
        } else if c == d {
            calc(c, a, b, e)
        } else {
            calc(d, a, b, c)
        };
        PokerHand::ONE_PAIR + val
    }

    fn value_four_kind(&self) -> u32 {
        let (a, b, .., e) = self.get_ranks();
        let val = if a == b {
            14_u32 * a + e
        } else {
            14_u32 * e + a
        };
        PokerHand::FOUR_OF_A_KIND + val
    }

    fn value_full_house(&self) -> u32 {
        let (a, b, c, d, e) = self.get_ranks();
        let val = if a == b && b == c {
            14_u32 * a + d
        } else {
            14_u32 * e + a
        };
        PokerHand::FULL_HOUSE + val
    }

    fn value_hight_card(&self) -> u32 {
        self.cards
            .iter()
            .enumerate()
            .map(|(i, &Card(rank, _))| 14_u32.pow(i as u32) * (rank as u32))
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn the_hand_is_flush() {
        let flush_hand = "3S 10S 5S JS QS";
        let hand = PokerHand::new(flush_hand).unwrap();
        assert!(hand.is_flush())
    }

    #[test]
    fn the_hand_is_straight() {
        let hand = PokerHand::new("7S JC 10D 8H 9S").unwrap();
        assert!(hand.is_straight());
        let hand = PokerHand::new("4S 5H 4C 8S 5D").unwrap();
        assert!(!hand.is_straight());
    }

    #[test]
    fn the_hand_is_straight_flush() {
        let hand = PokerHand::new("7S JS 10S 8S 9S").unwrap();
        assert!(hand.is_straight_flush())
    }

    #[test]
    fn the_hand_is_four_of_kind() {
        let hand = PokerHand::new("7S 7D 10S 7C 7H").unwrap();
        assert!(hand.is_4s())
    }

    #[test]
    fn the_hand_is_three_of_kind() {
        let hand = PokerHand::new("8S 7D 10S 7C 7H").unwrap();
        assert!(hand.is_3s())
    }

    #[test]
    fn the_hand_is_full_house() {
        let hand = PokerHand::new("8S 7D 8C 7C 7H").unwrap();
        assert!(hand.is_full_house())
    }

    #[test]
    fn the_hand_is_two_pairs() {
        let hand = PokerHand::new("4S 5H 4C 8S 5D").unwrap();
        assert!(hand.is_two_pairs())
    }

    #[test]
    fn the_hand_is_pair() {
        let hand = PokerHand::new("8S 5D 8C 7C 10H").unwrap();
        assert!(hand.is_pair())
    }
}
