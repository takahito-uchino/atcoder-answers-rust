use proconio::input;
use std::{cmp, collections::HashMap};

fn main() {
    input! {
        n: usize,
        k: i32,
        s: String,
    }

    let mut snum = HashMap::new();
    let mut tnum = HashMap::new();

    for c in s.chars() {
        *snum.entry(c).or_insert(0) += 1;
        *tnum.entry(c).or_insert(0) += 1;
    }

    let mut t = String::new();
    let mut l = 0;
    let chars = s.chars().collect::<Vec<char>>();

    for i in 0..n {
        let si = chars[i];
        *snum.entry(si).or_insert(0) -= 1;
        let mut found = false;

        for c in 'a'..='z' {
            if let Some(num) = tnum.get_mut(&c) {
                if *num == 0 {
                    continue;
                }
                let original_num = *num;
                *num -= 1;
                l += (c != si) as i32;
                if mincost(n - i - 1, &snum, &tnum) <= k - l {
                    t.push(c);
                    found = true;
                    break;
                }
                *num = original_num;
                l -= (c != si) as i32;
            }
        }

        if !found {
            t.push(si)
        }
    }

    println!("{}", t)
}

fn mincost(n: usize, snum: &HashMap<char, i32>, tnum: &HashMap<char, i32>) -> i32 {
    let mut result = 0;
    for c in 'a'..'z' {
        result += cmp::min(*snum.get(&c).unwrap_or(&0), *tnum.get(&c).unwrap_or(&0));
    }
    (n as i32) - result
}
