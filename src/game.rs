use crate::card::Card;
use crate::card::Value;

pub struct Game {
    pub dealer_cards: Vec<Card>,
    pub dealer_score: u8,
    pub player_cards: Vec<Card>,
    pub player_score: u8,
}
impl Game {
    pub fn new() -> Game {
        // initial bank cards
        let mut dealer_cards: Vec<Card> = Vec::new();
        dealer_cards.push(Card::random());
        dealer_cards.push(Card::random());
        let dealer_score: u8 = Game::compute_score(&dealer_cards);
        // initial player cards
        let mut player_cards: Vec<Card> = Vec::new();
        player_cards.push(Card::random());
        player_cards.push(Card::random());
        let player_score: u8 = Game::compute_score(&player_cards);
        // return randomly initialized game state
        Game {
            dealer_cards: dealer_cards,
            dealer_score: dealer_score,
            player_cards: player_cards,
            player_score: player_score,
        }
    }
    pub fn player_draw_card(&mut self) {
        self.player_cards.push(Card::random());
        self.player_score = Game::compute_score(&self.player_cards);
    }
    pub fn dealer_draw_card(&mut self) {
        self.dealer_cards.push(Card::random());
        self.dealer_score = Game::compute_score(&self.dealer_cards);
    }
    fn compute_score(cards: &Vec<Card>) -> u8 {
        let mut score = 0;
        let mut contains_ace: bool = false;
        for card in cards.iter() {
            let card_score: u8 = match card.value {
                Value::Two => 2,
                Value::Three => 3,
                Value::Four => 4,
                Value::Five => 5,
                Value::Six => 6,
                Value::Seven => 7,
                Value::Eight => 8,
                Value::Nine => 9,
                Value::Ten => 10,
                Value::Jack => 10,
                Value::Queen => 10,
                Value::King => 10,
                Value::Ace => {
                    contains_ace = true;
                    11
                },
            };
            score += card_score;
        }
        if score > 21 && contains_ace {
            score -= 10
        }
        score
    }
}
