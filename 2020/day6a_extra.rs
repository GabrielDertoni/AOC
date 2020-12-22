use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

struct SplitIter<T> {
    deque: VecDeque<T>
}

impl<T> SplitIter<T> {
    fn new(deque: VecDeque<T>) -> Self {
        SplitIter { deque }
    }
}

impl<T> Iterator for SplitIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.deque.pop_front()
    }
}

struct Split<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    iterator: I,
    predicate: P,
    finished: bool,
}

impl<I, P> Split<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    fn new(iterator: I, predicate: P, finished: bool) -> Self {
        Split { iterator, predicate, finished }
    }
}

impl<I, P> Iterator for Split<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    type Item = SplitIter<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished { return None };
        let mut curr_iter = VecDeque::new();
        while let Some(next) = self.iterator.next() {
            if !(self.predicate)(&next) {
                curr_iter.push_back(next);
            } else {
                return Some(SplitIter::new(curr_iter));
            }
        }
        self.finished = true;
        Some(SplitIter::new(curr_iter))
    }
}

trait Splittable<I: Iterator> {
    fn split<P>(self, predicate: P) -> Split<I, P>
        where P: FnMut(&I::Item) -> bool;
}

impl<I> Splittable<I> for I
where
    I: Iterator,
{
    #[inline]
    fn split<P>(self, predicate: P) -> Split<I, P>
    where
        P: FnMut(&I::Item) -> bool,
    {
        Split::new(self, predicate, false)
    }
}


fn main() {
    let stdin = io::stdin();
    let group_iter = stdin
        .lock()
        .lines()
        .map(Result::unwrap)
        .split(|line| line.len() == 0)
        .map(|group| group.fold(String::new(), |mut all, s| {
            all.push_str(&s);
            all
        }))
        .map(|answers| {
            let mut answered = [false; 26];
            answers.chars().for_each(|ans| answered[ans as usize - 97] = true);
            answered
        });

    let mut sum = 0;
    for group in group_iter {
        let count = group.iter().filter(|&el| *el).count();
        sum += count;
    }
    println!("Final sum is: {}", sum);
}
