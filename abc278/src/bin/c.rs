use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        _n: usize,
        q: usize,
        tab: [(usize, usize, usize); q],
    }

    let mut follow: HashMap<usize, HashMap<usize, bool>> = HashMap::new();

    for (t, a, b) in tab {
        match t {
            1 => {
                follow.entry(a).or_insert_with(HashMap::new).insert(b, true);
            }
            2 => {
                if let Some(followers) = follow.get_mut(&a) {
                    followers.insert(b, false);
                }
            }
            3 => {
                let a_follows_b = follow
                    .get(&a)
                    .and_then(|f| f.get(&b))
                    .cloned()
                    .unwrap_or(false);
                let b_follows_a = follow
                    .get(&b)
                    .and_then(|f| f.get(&a))
                    .cloned()
                    .unwrap_or(false);
                if a_follows_b && b_follows_a {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
            _ => {}
        }
    }
}
