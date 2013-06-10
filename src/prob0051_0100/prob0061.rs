#[link(name = "prob0061", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;
use std::vec;
use common::problem::{Problem};

pub static problem: Problem<'static> = Problem {
    id: 61,
    answer: "28684",
    solver: solve
};

fn create_map(f: &fn(uint) -> uint) -> ~[~[uint]] {
    let mut result = vec::from_elem(100, ~[]);
    let mut i = 1;
    loop {
        let n = f(i);
        if n >= 10000 { break; }
        result[n / 100].push(n % 100);
        i += 1;
    }
    return result;
}

pub fn solve() -> ~str {
    let map = ~[
        create_map(|n| n * (n + 1) / 2),
        create_map(|n| n * n),
        create_map(|n| n * (3 * n - 1) / 2),
        create_map(|n| n * (2 * n - 1)),
        create_map(|n| n * (5 * n - 3) / 2),
        create_map(|n| n * (3 * n - 2))
    ];

    let mut result = ~[];
    for vec::each_permutation([0u, 1u, 2u, 3u, 4u]) |idx| {
        for map[5].eachi |i, v5| {
            if i < 10 { loop; }
            for v5.each |&n5| {
                if n5 < 10 { loop; }
                for map[idx[0]][n5].each |&n0| {
                    if n0 < 10 { loop; }
                    for map[idx[1]][n0].each |&n1| {
                        if n1 < 10 { loop; }
                        for map[idx[2]][n1].each |&n2| {
                            if n2 < 10 { loop; }
                            for map[idx[3]][n2].each |&n3| {
                                if n3 < 10 { loop; }
                                for map[idx[4]][n3].each |&n4| {
                                    if n4 < 10 { loop; }
                                    if n4 == i {
                                        result.push(~[
                                            n5 * 100 + n0,
                                            n0 * 100 + n1,
                                            n1 * 100 + n2,
                                            n2 * 100 + n3,
                                            n3 * 100 + n4,
                                            n4 * 100 + n5
                                        ]);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let sum = result.foldl(0u, |s, vs| s + vs.foldl(0u, |s, &n| s + n));
    return sum.to_str();
}
