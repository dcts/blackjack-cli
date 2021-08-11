use rand::Rng;

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

#[derive(Debug, Clone)]
pub struct Card {
    pub value: Value,
    pub color: Color,
}
impl Card {
    // constructor
    pub fn new(value: Value, color: Color) -> Card {
        Card {
            value: value,
            color: color,
        }
    }
    pub fn random() -> Card {
        // pick random value
        let random_value: Value = match rand::thread_rng().gen_range(2..=14) {
            2 => Value::Two,
            3 => Value::Three,
            4 => Value::Four,
            5 => Value::Five,
            6 => Value::Six,
            7 => Value::Seven,
            8 => Value::Eight,
            9 => Value::Nine,
            10 => Value::Ten,
            11 => Value::Jack,
            12 => Value::Queen,
            13 => Value::King,
            14 => Value::Ace,
            _ => panic!("Randomly generated value is out of bound. Allowed 2-14."),
        };
        // pick random color
        let random_color: Color = match rand::thread_rng().gen_range(0..=3) {
            0 => Color::Heart,
            1 => Color::Diamond,
            2 => Color::Spade,
            3 => Color::Club,
            _ => panic!("Randomly generated value is out of bound. Allowed 0-3."),
        };
        // return card with randomly picked value and color
        Card::new(random_value, random_color)
    }

    pub fn to_string(&self) -> String {
        let card_draft = "┌─────┐\n\
                               │v    |\n\
                               │  c  |\n\
                               │    v|\n\
                               └─────┘";
        let mut card_string: String = String::from(card_draft);
        // inject values
        if self.value == Value::Ten {
            card_string = card_string.replace("v ", self.value_char());
            card_string = card_string.replace(" v", self.value_char());
        } else {
            card_string = card_string.replace("v", self.value_char());
        }
        // inject color
        card_string = card_string.replace("c", self.color_char());
        // return
        card_string
    }
    // return value char
    pub fn value_char(&self) -> &str {
        match self.value {
            Value::Two => "2",
            Value::Three => "3",
            Value::Four => "4",
            Value::Five => "5",
            Value::Six => "6",
            Value::Seven => "7",
            Value::Eight => "8",
            Value::Nine => "9",
            Value::Ten => "10",
            Value::Jack => "J",
            Value::Queen => "Q",
            Value::King => "K",
            Value::Ace => "A",
        }
    }
    // return color char
    pub fn color_char(&self) -> &str {
        match self.color {
            Color::Heart => "♥",
            Color::Diamond => "♦",
            Color::Spade => "♠",
            Color::Club => "♣",
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone)]
pub enum Color {
    Heart,
    Diamond,
    Spade,
    Club,
}
