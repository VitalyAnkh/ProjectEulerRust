#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase, slicing_syntax)]

#[phase(plugin, link)] extern crate common;
extern crate prime;

use prime::{Factorize, PrimeSet};

fn compute(max: uint) -> uint {
    let ps = PrimeSet::new();

    let abundant = range(2, max + 1)
        .filter(|&n| n.sum_of_proper_divisor(&ps) > n)
        .collect::<Vec<_>>();

    let mut sum_of_sum_abundant = 0;

    let mut is_sum_abundant = Vec::from_elem(max + 1, false);

    for (i, &a) in abundant.iter().enumerate() {
        for &b in abundant[i..].iter() {
            let s = a + b;
            if s > max { break; }
            if !is_sum_abundant[s] {
                sum_of_sum_abundant += s;
                is_sum_abundant[s] = true;
            }
        }
    }

    let sum_of_all_num = (1 + max) * max / 2;
    sum_of_all_num - sum_of_sum_abundant
}

fn solve() -> String {
    compute(28123).to_string()
}

problem!("4179871", solve);
