use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut count = HashMap::new();

    for str in s {
        let entry = count.entry(str.clone()).or_insert(0);
        if *entry == 0 {
            println!("{}", str);
        } else {
            println!("{}({})", str, entry);
        }
        *entry += 1;
    }
}
