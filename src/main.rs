mod card;

use card::Deck;

fn main() {
    let mut my_deck = Deck::new();
    for card in my_deck.cards() {
        println!("{}", card);
    }

    my_deck.shuffle();

    println!("shuffling");
    for card in my_deck.cards() {
        println!("{}", card);
    }
}
