use rand::Rng;

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
