use im_rc::HashMap;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut map = HashMap::new();

    for c in &s {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }

    for i in 1..=s.len() {
        let mut c = 0;
        for v in map.values() {
            if v == &i {
                c += 1;
            }
        }
        if c != 0 && c != 2 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
