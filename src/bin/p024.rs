#![warn(unused, bad_style,
        unnecessary_qualification, unnecessary_typecast, unused_result)]

extern crate num;

extern crate common;
extern crate integer;

use num::Integer as SInteger;
use common::Solver;
use integer::Integer;

fn compute(mut idx: uint, mut set: Vec<uint>) -> uint {
    let mut result = vec![];
    while set.len() > 0 {
        let perm = (set.len() - 1).factorial();
        let (rm_idx, rest) = idx.div_rem(&perm);
        idx = rest;
        result.push(set.remove(rm_idx).unwrap());
    }
    Integer::from_digits(result.into_iter().rev(), 10)
}

fn solve() -> String {
    compute(1000000 - 1, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]).to_string()
}

fn main() { Solver::new("2783915460", solve).run(); }

#[cfg(test)]
mod tests {
    #[test]
    fn four() {
        assert_eq!(12, super::compute(0, vec![0, 1, 2]));
        assert_eq!(21, super::compute(1, vec![0, 1, 2]));
        assert_eq!(102, super::compute(2, vec![0, 1, 2]));
        assert_eq!(120, super::compute(3, vec![0, 1, 2]));
        assert_eq!(201, super::compute(4, vec![0, 1, 2]));
        assert_eq!(210, super::compute(5, vec![0, 1, 2]));
    }
}