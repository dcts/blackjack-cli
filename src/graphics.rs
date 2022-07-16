use std::io;
use std::io::Write;
use std::{thread, time};

use crate::game::Game;
use crate::card::Card;
use crate::card::Value;

// 1,5 seconds wait time for dealer (for the graphics "engige")
const SLEEP_TIME: u64 = 1500;

// GRAPHICS "ENGINE"
pub fn sleep() {
    thread::sleep(time::Duration::from_millis(SLEEP_TIME));
}

pub fn print_title() {
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
    println!("=== BLACKJACK ===");
}

pub fn print_game_state_with_hidden_card(game: &Game) {
    print_title();
    println!("Dealer Score: ?");
    print_cards_hidden(&game.dealer_cards);
    println!("Player Score: {}", game.player_score);
    print_cards(&game.player_cards);
}

pub fn print_game_state(game: &Game, message: &str) {
    print_title();
    println!("Dealer Score: {}", game.dealer_score);
    print_cards(&game.dealer_cards);
    println!("Player Score: {}", game.player_score);
    print_cards(&game.player_cards);
    println!("{}", message);
}

pub fn print_game_end(game: &Game, final_message: &str) {
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

pub fn print_cards(cards: &Vec<Card>) {
    // generate card splits
    let mut card_splits: Vec<Vec<String>> = vec![];
    for card in cards {
        let card_string = String::from(card.to_string());
        let vec_temporary: Vec<&str> = card_string.split("\n").collect();
        // CMON THIS IS INSANE...
        // THERE HAS TO BE A BETTER
        // OF DOING THIS SHIT....
        // => encode and decode matrix
        // => e.g. [
        //           [card1_1, card2_1, card3_1, card3_1],
        //           [card1_2, card2_2, card3_2, card3_2],
        //           [card1_3, card2_3, card3_3, card3_3],
        //           [card1_4, card2_4, card3_4, card3_4],
        //           [card1_5, card2_5, card3_5, card3_5],
        //         ]
        // Create 2 functions
        // => fn cards_to_cardsmatrix(vec: Vec<Cards>) -> CardsMatrix
        // => fn cardsmatrix_to_cards(cardsmatrix: CardsMatrix) -> Cards

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

pub fn print_cards_hidden(cards: &Vec<Card>) {
    let cards_draft = "┌─────┐┌─────┐\n\
                             │░░░░░|│v    |\n\
                             │░░░░░|│  c  |\n\
                             │░░░░░|│    v|\n\
                             └─────┘└─────┘\n";
    let mut cards_string: String = String::from(cards_draft);

    // inject values
    match cards[0].value {
        Value::Ten => {
            cards_string = cards_string.replace("v ", cards[0].value_str());
            cards_string = cards_string.replace(" v", cards[0].value_str());
        },
        _ => {
            cards_string = cards_string.replace("v", cards[0].value_str());
            cards_string = cards_string.replace("v", cards[0].value_str());
        }
    }

    // inject color
    cards_string = cards_string.replace("c", cards[0].color_str());
    println!("{}", cards_string);
}

// INTEACTION INTERFACE
pub fn prompt_for_user_action(game: &Game) -> String {
    println!("Your score is {}. What do you like to do?\n- draw another card (d)\n- stop (s)", game.player_score);
    print!("> ");
    io::stdout().flush().unwrap();
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    choice.trim().to_string()
}
