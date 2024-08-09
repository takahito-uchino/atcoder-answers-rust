use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut s_count = HashMap::new();
    let mut t_count = HashMap::new();

    for c in s {
        let count = s_count.entry(c).or_insert(0);
        *count += 1;
    }

    for c in t {
        let count = t_count.entry(c).or_insert(0);
        *count += 1;
    }

    for c in "atcoder".chars() {
        let sc = *s_count.get(&c).unwrap_or(&0);
        let tc = *t_count.get(&c).unwrap_or(&0);
        let m = sc.max(tc);

        if s_count.get(&'@').unwrap_or(&0) < &(m - sc)
            || t_count.get(&'@').unwrap_or(&0) < &(m - tc)
        {
            println!("No");
            return;
        }

        *s_count.entry('@').or_insert(0) -= m - sc;
        *s_count.entry(c).or_insert(0) = m;
        *t_count.entry('@').or_insert(0) -= m - tc;
        *t_count.entry(c).or_insert(0) = m;
    }

    println!("{}", if s_count == t_count { "Yes" } else { "No" })
}
