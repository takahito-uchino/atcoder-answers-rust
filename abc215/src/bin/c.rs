use permutohedron::LexicalPermutation;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        k: usize,
    }

    s.sort_unstable();

    if k == 1 {
        println!("{}", s.iter().collect::<String>());
        return;
    }

    let mut count = 1;

    while s.next_permutation() {
        count += 1;
        if k == count {
            break;
        }
    }

    println!("{}", s.iter().collect::<String>())
}
