mod card;
mod score_hand;

use card::Deck;
use score_hand::score_hand;

fn main() {
    let mut deck = Deck::new();

    let hand = deck.draw_n(4).unwrap();
    let starter = deck.draw().unwrap();

    println!("hand: {:#?}", hand);
    println!("starter: {:#?}", starter);

    let score = score_hand(&hand, &starter);
    println!("score: {}", score);
}
