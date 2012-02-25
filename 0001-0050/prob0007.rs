use std;

fn isqrt(n: u64) -> u64 {
    let (min, max) = (0u64, n);
    while min < max {
        let mid = (min + max + 1u64) / 2u64;
        if (mid * mid) == n {
            ret mid;
        } else if (mid * mid) >= n {
            max = mid - 1u64;
        } else {
            min = mid;
        }
    }
    ret min;
}

fn gen_prime(&primes: [u64]) {
    let num = alt vec::last(primes) {
      none       { primes =  [2u64]; ret }
      some(2u64) { primes += [3u64]; ret }
      some(x)    { x + 2u64 }
    };
    while true {
        for p in primes {
            if p * p > num {
                primes += [num];
                ret;
            }
            if num % p == 0u64 {
                break;
            }
        }
        num += 2u64;
    }
    fail;
}

fn main() {
    let idx = 10000u64;
    let primes = [];
    uint::range(0u64, idx + 1u64) { |_num|
        gen_prime(primes);
    }
    std::io::println(#fmt("%u", primes[idx]));
}