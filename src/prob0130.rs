#[crate_id = "prob0130"];
#[crate_type = "rlib"];

extern mod math;
extern mod prob0129;

use std::iter;
use std::iter::AdditiveIterator;
use math::prime::Prime;

pub static EXPECTED_ANSWER: &'static str = "149253";

pub fn solve() -> ~str {
    let ps = Prime::new();
    iter::count(3u, 2)
        .filter(|&n| !n.is_multiple_of(&5))
        .filter(|&n| !ps.contains(n))
        .filter(|&n| (n - 1).is_multiple_of(&prob0129::a(n)))
        .take(25)
        .sum()
        .to_str()
}

#[cfg(test)]
mod test {
    use std::iter;
    use math::prime::Prime;
    use prob0129;

    #[test]
    fn first5() {
        let ps = Prime::new();
        let mut it = iter::count(3u, 2)
            .filter(|&n| !n.is_multiple_of(&5))
            .filter(|&n| !ps.contains(n))
            .filter(|&n| (n - 1).is_multiple_of(&prob0129::a(n)));

        assert_eq!(Some(91), it.next());
        assert_eq!(Some(259), it.next());
        assert_eq!(Some(451), it.next());
        assert_eq!(Some(481), it.next());
        assert_eq!(Some(703), it.next());
    }
}
