
/**
 *
 *
 */

use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

type Card = usize;

enum Winner {
    Player1(VecDeque<Card>),
    Player2(VecDeque<Card>),
}

fn get_hash(deck: &VecDeque<Card>) -> u64 {
    let mut hasher = DefaultHasher::new();
    deck.hash(&mut hasher);
    hasher.finish()
}

fn play_game(mut p1_deck: VecDeque<Card>, mut p2_deck: VecDeque<Card>, depth: usize) -> Winner {
    // let p1_starting = p1_deck.clone();
    // let p2_starting = p2_deck.clone();
    let mut p1_hashes = HashSet::new();
    let mut p2_hashes = HashSet::new();
    println!("Game hash in depth {} is {:?} {:?}", depth, p1_deck, p2_deck);

    let mut i = 0;
    while !p1_deck.is_empty() && !p2_deck.is_empty() {
        if p1_hashes.contains(&p1_deck) || p2_hashes.contains(&p2_deck) {
            println!("Here with i = {}, depth is {}", i, depth);

            return Winner::Player1(p1_deck);
        }
        p1_hashes.insert(p1_deck.clone());
        p2_hashes.insert(p2_deck.clone());
        /*
        if p1_hashes.contains(&get_hash(&p1_deck)) || p2_hashes.contains(&get_hash(&p2_deck)) {
            println!("Here with i = {}, depth is {}", i, depth);

            return Winner::Player1(p1_deck);
        }
        p1_hashes.insert(get_hash(&p1_deck));
        p2_hashes.insert(get_hash(&p2_deck));
        */

        let p1_card = p1_deck.pop_front().unwrap();
        let p2_card = p2_deck.pop_front().unwrap();

        let p1_winner = if p1_deck.len() < p1_card || p2_deck.len() < p2_card { p1_card > p2_card }
                        else if let Winner::Player1(_) = play_game(p1_deck.clone(), p2_deck.clone(), depth + 1) { true }
                        else { false };

        if p1_winner {
            p1_deck.push_back(p1_card);
            p1_deck.push_back(p2_card);
        } else {
            p2_deck.push_back(p2_card);
            p2_deck.push_back(p1_card);
        }
        i += 1;
    }
    if p1_deck.is_empty() {
        Winner::Player2(p2_deck)
    } else {
        Winner::Player1(p1_deck)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines_iter = stdin.lock()
        .lines()
        .map(Result::unwrap);

    let p1_cards: VecDeque<Card> = lines_iter.by_ref()
        .take_while(|line| line.len() > 0)
        .skip(1) // Skip "Player 1:"
        .map(|line| line.parse().unwrap())
        .collect();

    let p2_cards: VecDeque<Card> = lines_iter.by_ref()
        .skip(1) // Skip "Player 2:"
        .map(|line| line.parse().unwrap())
        .collect();


    let mut winner_name: &'static str = "Player 1";
    let winner = match play_game(p1_cards, p2_cards, 0) {
        Winner::Player1(deck) => deck,
        Winner::Player2(deck) => {
            winner_name = "Player 2";
            deck
        },
    };
    let final_score: usize = winner.iter()
        .zip((1..=winner.len()).rev())
        .map(|(&card, value)| card * value)
        .sum();

    println!("The winner is {} with a final score of {}", winner_name, final_score);
}
