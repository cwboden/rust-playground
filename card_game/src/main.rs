mod deck;

fn main() {
    let deck_name = String::from("My Deck");
    let mut my_deck = deck::Deck::new(&deck_name);

    const SUITS: [deck::Suit; 4] = [
            deck::Suit::Clubs, 
            deck::Suit::Diamonds, 
            deck::Suit::Hearts, 
            deck::Suit::Spades];

    const RANKS: [deck::Rank; 13] = [
            deck::Rank::Ace,
            deck::Rank::Two,
            deck::Rank::Three,
            deck::Rank::Four,
            deck::Rank::Five,
            deck::Rank::Six,
            deck::Rank::Seven,
            deck::Rank::Eight,
            deck::Rank::Nine,
            deck::Rank::Ten,
            deck::Rank::Jack,
            deck::Rank::Queen,
            deck::Rank::King];

    for suit in SUITS.iter() {
        for rank in RANKS.iter() {
            let card = deck::Card::new(*rank, *suit);
            my_deck.add_card(card);
        }
    }

    println!("Original Deck");
    my_deck.print();

    println!("Shuffled Deck");
    my_deck.shuffle_rng();
    my_deck.print();
}
