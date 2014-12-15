#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

extern crate common;
extern crate num;
extern crate cont_frac;

use std::str::FromStr;
use num::{One, Integer, BigUint};
use common::Solver;
use cont_frac::PelNegRoots;

// b/s * (b-1)/(s-1) = 1/2
// 2b(b - 1) = s * (s-1)
// 2b^2 - 2b = s^2 - s
// 2(b - 1/2)^2 - 1/2 = (s - 1/2)^2 - 1/4
// 2(2b - 1)^2 - 2 = (2s - 1)^2 - 1
// (2s - 1)^2 - 2(2b - 1)^2 = -1
// x^2 - 2y = -1
// s = (x + 1) / 2
// b = (y + 1) / 2
fn compute(limit: BigUint) -> BigUint {
    let one = One::one();
    PelNegRoots::<BigUint>::new(2)
        .filter(|&(ref x, ref y)| x.is_odd() && y.is_odd())
        .map(|(x, y)| ((x + one) >> 1, (y + one) >> 1))
        .find(|&(ref x, ref _y)| ((*x) >= limit))
        .map(|(_x, y)| y)
        .unwrap()
}

fn solve() -> String {
    let limit = FromStr::from_str("1000000000000").unwrap();
    compute(limit).to_string()
}

fn main() {
    Solver::new("756872327473", solve).run();
}

#[cfg(test)]
mod tests {
    use num::BigUint;

    #[test]
    fn twenty_one() {
        fn check(result: uint, total: uint) {
            let result: BigUint = FromPrimitive::from_uint(result).unwrap();
            let total: BigUint = FromPrimitive::from_uint(total).unwrap();
            assert_eq!(result, super::compute(total));
        }
        check(15, 21);
        check(85, 22);
    }
}