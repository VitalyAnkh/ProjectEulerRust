#![crate_name = "prob0078"]
#![crate_type = "rlib"]

use std::iter;

pub const EXPECTED_ANSWER: &'static str = "55374";

const MILLION: int = 1000000;

#[inline]
fn penta(n: int) -> int { n * (3 * n - 1) / 2 }

pub fn solve() -> String {
    let mut v = [0, ..65536];
    v[0] = 1;

    for n in iter::count(1i, 1) {
        let mut way = 0;
        for i in iter::count(0i, 1) {
            let k = i % 4;
            let p = if k == 0 || k == 2 { penta(i / 2 + 1) } else { penta(-i / 2 - 1) };
            if p > n { break; }

            way = match k {
                0 => way + v[(n - p) as uint],
                1 => way + v[(n - p) as uint],
                2 => way - v[(n - p) as uint],
                _ => way - v[(n - p) as uint]
            } % MILLION
        }
        v[n as uint] = way;

        if way == 0 { return n.to_string() }
    }

    unreachable!()
}
