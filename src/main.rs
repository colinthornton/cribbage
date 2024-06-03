mod card;

use card::Deck;

fn main() {
    let mut deck = Deck::new();

    let hand = deck.draw_n(6).unwrap();
    let other_hand = deck.draw_n(6).unwrap();

    for card in hand.iter() {
        println!("{}", card);
    }
    for card in other_hand.iter() {
        println!("{}", card);
    }
}
