use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut index_map = HashMap::new();

    for i in 0..n {
        index_map.insert(a[i], i);
    }

    let mut k = 0;
    let mut l = Vec::new();

    for i in 0..n {
        if a[i] != i + 1 {
            let index = index_map[&(i + 1)];
            k += 1;
            l.push((i + 1, index + 1));
            a.swap(i, index);
            index_map.insert(a[i], i);
            index_map.insert(a[index], index);
        }
    }

    println!("{}", k);

    for (a, b) in l {
        println!("{} {}", a, b);
    }
}
