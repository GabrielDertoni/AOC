use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

type Card = u32;

fn main() {
    let stdin = io::stdin();
    let mut lines_iter = stdin.lock()
        .lines()
        .map(Result::unwrap);

    let mut p1_cards: VecDeque<Card> = lines_iter.by_ref()
        .take_while(|line| line.len() > 0)
        .skip(1) // Skip "Player 1:"
        .map(|line| line.parse().unwrap())
        .collect();

    let mut p2_cards: VecDeque<Card> = lines_iter.by_ref()
        .skip(1) // Skip "Player 2:"
        .map(|line| line.parse().unwrap())
        .collect();

    while !p1_cards.is_empty() && !p2_cards.is_empty() {
        let p1_plays = p1_cards.pop_front().unwrap();
        let p2_plays = p2_cards.pop_front().unwrap();

        if p1_plays > p2_plays {
            p1_cards.push_back(p1_plays);
            p1_cards.push_back(p2_plays);
        } else {
            p2_cards.push_back(p2_plays);
            p2_cards.push_back(p1_plays);
        }
    }

    let winner = if p1_cards.is_empty() { &p2_cards } else { &p1_cards };
    let final_score: usize = winner.iter()
        .zip((1..=winner.len()).rev())
        .map(|(&card, value)| card as usize * value)
        .sum();

    println!( "The winner is {} with a final score of {}"
            , if p1_cards.is_empty() { "Player 2" } else { "Player 1" }
            , final_score
            );
}
