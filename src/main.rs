use card::Deck;
use the_show::score_the_show;

mod card;
mod the_play;
mod the_show;

fn main() {
    let mut deck = Deck::new();

    let hand = deck.draw_n(4).unwrap();
    let starter = deck.draw().unwrap();

    println!("hand: {:#?}", hand);
    println!("starter: {:#?}", starter);

    let score = score_the_show(&hand, &starter);
    println!("score: {}", score);
}
