use crate::primes;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::ops::Not;

pub struct Anagram {
    identifier: BigUint,
    letters: std::collections::HashMap<char, u32>,
}

impl Anagram {
    fn identify(letters: &std::collections::HashMap<char, u32>, s: &str) -> Option<BigUint> {
        s.chars().try_fold(One::one(), |acc, x| -> Option<BigUint> {
            Some(acc * letters.get(&x)?)
        })
    }

    pub fn search_candidate<T: Iterator<Item = String>>(
        &self,
        word_list: T,
    ) -> Vec<(String, BigUint)> {
        word_list
            .filter_map(|s| {
                let id = Self::identify(&self.letters, &s)?;
                if &self.identifier % &id == Zero::zero() {
                    Some((s, id))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn search<T: Iterator<Item = String>>(&self, word_list: T) -> Vec<Vec<String>> {
        let candidates = self.search_candidate(word_list);
        let mut anagram_words = Vec::<Vec<String>>::new();
        let mut stack: Vec<(&[(String, BigUint)], BigUint)> = Vec::<_>::new();
        let mut num: BigUint = self.identifier.clone();
        let mut candidates = candidates.as_slice();
        loop {
            if num == One::one() {
                anagram_words.push(stack.iter().map(|x| x.0[0].0.clone()).collect());
                if let Some((r, n)) = stack.pop() {
                    candidates = &r[1..];
                    num = n;
                } else {
                    break;
                }
            } else {
                match candidates {
                    [] => {
                        if let Some((r, n)) = stack.pop() {
                            candidates = &r[1..];
                            num = n;
                        } else {
                            break;
                        }
                    }
                    [(_, ref id), ..] if &num % id == Zero::zero() => {
                        stack.push((candidates, num.clone()));
                        num /= id;
                    }
                    [_, ..] => {
                        candidates = &candidates[1..];
                    }
                }
            }
        }
        anagram_words
    }

    pub fn new(source: &str) -> Self {
        let mut p_itr = primes::PrimeIter::new();
        let mut letters = std::collections::HashMap::<char, u32>::new();
        for c in source.chars() {
            if letters.contains_key(&c).not() {
                letters.insert(c, p_itr.next().unwrap());
            }
        }
        let id = Self::identify(&letters, source).unwrap();
        Anagram {
            letters,
            identifier: id,
        }
    }
}
