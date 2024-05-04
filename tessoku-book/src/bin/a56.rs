use proconio::{input, marker::Chars};

const MOD: i64 = 2147483647;

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        queries: [(usize, usize, usize, usize); q],
    }

    let t = s.iter().map(|&c| (c as i64 - 'a' as i64 + 1)).collect::<Vec<i64>>();

    let mut power100 = vec![0; n + 1];
    power100[0] = 1;

    for i in 0..n {
        power100[i + 1] = power100[i] * 100 % MOD;
    }

    let mut h = vec![0; n + 1];
    for i in 0..n {
        h[i + 1] = (h[i] * 100 + t[i]) % MOD;
    }

    fn hash_value(h: &Vec<i64>, power100: &Vec<i64>, l: usize, r: usize) -> i64 {
        let value = (h[r] - h[l - 1] * power100[r - l + 1]) % MOD;
        if value < 0 { value + MOD } else { value }
    }

    for (a, b, c, d) in queries {
        let hash1 = hash_value(&h, &power100, a, b);
        let hash2 = hash_value(&h, &power100, c, d);
        if hash1 == hash2 {
            println!("Yes")
        } else {
            println!("No")
        }
    }
}
