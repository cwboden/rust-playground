use rand;
use rand::seq::SliceRandom;

#[derive(Copy, Clone, Debug)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Copy, Clone, Debug)]
pub enum Rank {
    Ace,
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
}

pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card {
            rank,
            suit,
        }
    }

    pub fn print(&self) {
        println!("{:?} of {:?}", self.rank, self.suit);
    }
}

pub struct Deck {
    cards: Vec<Card>,
    name: String,
}

impl Deck {
    pub fn new(name: &String) -> Deck {
        Deck {
            cards: Vec::new(),
            name: String::from(name),
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn print(&self) {
        println!("{}", self.name);
        
        for card in self.cards.iter() {
            card.print()
        }
    }

    pub fn shuffle_rng(&mut self) {
        let mut rng = rand::thread_rng();        
        self.cards.shuffle(&mut rng);
    }
}
