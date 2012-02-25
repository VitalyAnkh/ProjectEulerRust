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

fn gen_prime(primes: [u64]) -> [u64] {
    let num = alt vec::last(primes) {
      none       { ret [2u64] }
      some(2u64) { ret primes + [3u64] }
      some(x)    { x + 2u64 }
    };
    while true {
        for p in primes {
            if p * p > num {
                ret primes + [num];
            }
            if num % p == 0u64 {
                break;
            }
        }
        num += 2u64;
    }
    fail;
}

fn div_mult(&num: u64, f: u64) -> u64 {
    let exp = 0u64;
    while (num % f == 0u64) {
        exp += 1u64;
        num /= f;
    }
    ret exp;
}

fn factorize(num: u64, &primes: [u64]) -> [(u64, u64)] {
    let itr = num;
    let result = [];

    for p in primes {
        let exp = div_mult(itr, p);
        if exp > 0u64 {
            result += [(p, exp)];
        }
    }

    while itr != 1u64 {
        primes = gen_prime(primes);
        let p = vec::last_total(primes);
        let exp = div_mult(itr, p);
        if exp > 0u64 {
            result += [(p, exp)];
        }
    }

    ret result;
}

fn merge_fact(fs1: [(u64, u64)], fs2: [(u64, u64)]) -> [(u64, u64)] {
    let result = [];
    let i1 = 0u, i2 = 0u;
    let len1 = vec::len(fs1), len2 = vec::len(fs2);
    while (i1 < len1 && i2 < len2) {
        let (base1, exp1) = fs1[i1];
        let (base2, exp2) = fs2[i2];
        if (base1 < base2) {
            result += [(base1, exp1)];
            i1 += 1u64;
        } else if (base1 > base2) {
            result += [(base2, exp2)];
            i2 += 1u64;
        } else {
            result += [(base1, uint::max(exp1, exp2))];
            i1 += 1u64;
            i2 += 1u64;
        }
    }
    if i1 < len1 {
        result += vec::slice(fs1, i1, len1);
    }
    if i2 < len2 {
        result += vec::slice(fs2, i2, len2);
    }
    ret result;
}

fn merge_facti(fss: [[(u64, u64)]]) -> [(u64, u64)] {
    ret alt vec::len(fss) {
      0u64 { [] }
      1u64 { fss[0] }
      l    {
        let pre  = merge_facti(vec::slice(fss, 0u64, l / 2u64));
        let post = merge_facti(vec::slice(fss, l / 2u64, l));
        merge_fact(pre, post)
      }
    }
}

fn pow(base: u64, exp: u64) -> u64 {
    let result = 1u64;
    let itr = exp;
    let pow = base;
    while itr > 0u64 {
        if itr & 0x1u64 == 0x1u64 {
            result *= pow;
        }
        itr >>= 1u64;
        pow *= pow;
    }
    ret result;
}

fn fact_to_uint(fs: [(u64, u64)]) -> u64 {
    let result = 1u64;
    for (base, exp) in fs {
        result *= pow(base, exp);
    }
    ret result;
}

fn main() {
    let primes = [];
    let factors = vec::map(vec::enum_uints(1u64, 20u64)) { |num| factorize(num, primes) };
    std::io::println(#fmt("%u", fact_to_uint(merge_facti(factors))));
}