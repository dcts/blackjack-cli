use rand::Rng;
use std::io;
use std::io::Write;
use std::{thread, time};

// 1,5 seconds wait time for dealer (for the graphics "engige")
const SLEEP_TIME: u64 = 1500;

pub fn main() {
    // create new Game
    let mut game: Game = Game::new();

    // display initial game state
    print_game_state_with_hidden_card(&game);

    // endgame condition breaks loop
    loop {
        // prompt user action
        let choice: String = prompt_for_user_action(&game);
        if choice == "d" {
            game.player_draw_card();
            print_game_state_with_hidden_card(&game);
            if game.player_score > 21 {
                print_game_end(&game, "❌ BUSTED ❌");
                break;
            }

        } else if choice == "s" {
            // check if dealer has won
            if game.dealer_score > game.player_score {
                print_game_end(&game, "❌ YOU LOST ❌");
                break;
            } else if game.dealer_score == game.player_score {
                print_game_end(&game, "✋ It's a DRAW ✋");
                break;
            } else {
                print_game_state(&game, "👀 Dealer revealer his hidden card...\n...\n...");
                while game.dealer_score < game.player_score {
                    // wait 1 sec
                    sleep();
                    // take another card and display
                    game.dealer_draw_card();
                    // check endgame conditions
                    if game.dealer_score > 21 {
                        print_game_end(&game, "🎉 YOU WON 🎉");
                        break;
                    } else if game.dealer_score == game.player_score {
                        print_game_end(&game, "✋ It's a DRAW ✋");
                        break;
                    } else if game.dealer_score > game.player_score {
                        print_game_end(&game, "❌ YOU LOST ❌");
                        break;
                    }
                    print_game_state(&game, "🃏 Dealer has grabbed another card... \n...\n...");
                }
                break;
            }
        } else {
            println!("(INVALID INPUT)");
        }
    }
}

struct Game {
    dealer_cards: Vec<Card>,
    dealer_score: u8,
    player_cards: Vec<Card>,
    player_score: u8,
}
impl Game {
    fn new() -> Game {
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
    fn player_draw_card(&mut self) {
        self.player_cards.push(Card::random());
        self.player_score = Game::compute_score(&self.player_cards);
    }
    fn dealer_draw_card(&mut self) {
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
struct Card {
    value: Value,
    color: Color,
}
impl Card {
    // constructor
    fn new(value: Value, color: Color) -> Card {
        Card {
            value: value,
            color: color,
        }
    }
    fn random() -> Card {
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

    fn to_string(&self) -> String {
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
    fn value_char(&self) -> &str {
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
    fn color_char(&self) -> &str {
        match self.color {
            Color::Heart => "♥",
            Color::Diamond => "♦",
            Color::Spade => "♠",
            Color::Club => "♣",
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Value {
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
enum Color {
    Heart,
    Diamond,
    Spade,
    Club,
}

// GRAPHICS "ENGINE"
fn sleep() {
    thread::sleep(time::Duration::from_millis(SLEEP_TIME));
}

fn print_title() {
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("=== BLACKJACK ===");
}

fn print_game_state_with_hidden_card(game: &Game) {
    print_title();
    println!("Dealer Score: ?");
    print_cards_hidden(&game.dealer_cards);
    println!("Player Score: {}", game.player_score);
    print_cards(&game.player_cards);
}

fn print_game_state(game: &Game, message: &str) {
    print_title();
    println!("Dealer Score: {}", game.dealer_score);
    print_cards(&game.dealer_cards);
    println!("Player Score: {}", game.player_score);
    print_cards(&game.player_cards);
    println!("{}", message);
}

fn print_game_end(game: &Game, final_message: &str) {
    // print game status to console
    print_title();
    println!("Dealer Score: {}", game.dealer_score);
    print_cards(&game.dealer_cards);
    println!("Player Score: {}", game.player_score);
    print_cards(&game.player_cards);

    // to make the end status have the same dimensions
    // as an ongoing game I am adding some random lines
    println!("{}\n- Your score is {}\n- The Dealer's score is {}", final_message, game.player_score, game.dealer_score);
    print!("> press any key to exit");
    io::stdout().flush().unwrap();
    let mut choice = String::new();
    // prompt user to press any key to exit program
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");
    println!("...exiting");
}

fn print_cards(cards: &Vec<Card>) {
    // generate card splits
    let mut card_splits: Vec<Vec<String>> = vec![];
    for card in cards {
        let card_string = String::from(card.to_string());
        let vec_temporary: Vec<&str> = card_string.split("\n").collect();
        // CMON THIS IS INSANE...
        // THERE HAS TO BE A BETTER
        // OF DOING THIS SHIT....
        let mut vec: Vec<String> = vec![];
        for temp in vec_temporary {
            vec.push(temp.to_owned());
        }
        card_splits.push(vec);
    }

    // iterate over each line
    let mut final_string = String::from("");
    for line in 1..=5 {
        for split in card_splits.iter() {
            final_string.push_str(&split[line-1]);
        }
        final_string.push_str("\n");
    }
    println!("{}",final_string);
}

fn print_cards_hidden(cards: &Vec<Card>) {
    let cards_draft = "┌─────┐┌─────┐\n\
                            │░░░░░|│v    |\n\
                            │░░░░░|│  c  |\n\
                            │░░░░░|│    v|\n\
                            └─────┘└─────┘\n";
    let mut cards_string: String = String::from(cards_draft);
    // inject values
    if cards[0].value == Value::Ten {
        cards_string = cards_string.replace("v ", cards[0].value_char());
        cards_string = cards_string.replace(" v", cards[0].value_char());
    } else {
        cards_string = cards_string.replace("v", cards[0].value_char());
        cards_string = cards_string.replace("v", cards[0].value_char());
    }
    // inject color
    cards_string = cards_string.replace("c", cards[0].color_char());
    // return
    println!("{}", cards_string);
}

// INTEACTION INTERFACE
fn prompt_for_user_action(game: &Game) -> String {
    println!("Your score is {}. What do you like to do?\n- draw another card (d)\n- stop (s)", game.player_score);
    print!("> ");
    io::stdout().flush().unwrap();
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    choice.trim().to_string()
}
