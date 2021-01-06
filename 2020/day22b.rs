use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;
use std::collections::HashSet;

type Card = usize;

enum Winner {
    Player1(VecDeque<Card>),
    Player2(VecDeque<Card>),
}

fn play_game(mut p1_deck: VecDeque<Card>, mut p2_deck: VecDeque<Card>) -> Winner {
    let mut p1_hashes = HashSet::new();
    let mut p2_hashes = HashSet::new();

    while !p1_deck.is_empty() && !p2_deck.is_empty() {
        if p1_hashes.contains(&p1_deck) || p2_hashes.contains(&p2_deck) {
            return Winner::Player1(p1_deck);
        }
        p1_hashes.insert(p1_deck.clone());
        p2_hashes.insert(p2_deck.clone());

        let p1_card = p1_deck.pop_front().unwrap();
        let p2_card = p2_deck.pop_front().unwrap();

        let p1_winner = if p1_deck.len() < p1_card || p2_deck.len() < p2_card {
            p1_card > p2_card
        } else {
            let p1_cloned = p1_deck.iter()
                .take(p1_card)
                .map(ToOwned::to_owned)
                .collect();

            let p2_cloned = p2_deck.iter()
                .take(p2_card)
                .map(ToOwned::to_owned)
                .collect();

            match play_game(p1_cloned, p2_cloned) {
                Winner::Player1(_) => true,
                Winner::Player2(_) => false,
            }
        };

        if p1_winner {
            p1_deck.push_back(p1_card);
            p1_deck.push_back(p2_card);
        } else {
            p2_deck.push_back(p2_card);
            p2_deck.push_back(p1_card);
        }
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


    let (winner, deck) = match play_game(p1_cards, p2_cards) {
        Winner::Player1(deck) => ("Player 1", deck),
        Winner::Player2(deck) => ("Player 2", deck),
    };
    let final_score: usize = deck.iter()
        .zip((1..=deck.len()).rev())
        .map(|(&card, value)| card * value)
        .sum();

    println!("The winner is {} with a final score of {}", winner, final_score);
}
