use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [isize; n],
        bk: [(isize, usize); q],
    }

    a.sort_unstable();

    for (b, k) in bk {
        match find_nth_closest(&a, b, k) {
            Some(value) => println!("{}", (value - b).abs()),
            None => println!("Err"),
        }
    }
}

fn find_nth_closest(arr: &[isize], target: isize, k: usize) -> Option<isize> {
    let _index = arr
        .binary_search_by(|&x| compare_with_target(x, target))
        .unwrap_or_else(|x| x);

    let mut diffs: Vec<(isize, isize)> = arr.iter().map(|&x| (x, (x - target).abs())).collect();

    diffs.sort_by_key(|&(_, diff)| diff);

    diffs.get(k - 1).map(|&(value, _)| value)
}

fn compare_with_target(value: isize, target: isize) -> Ordering {
    if value < target {
        Ordering::Less
    } else if value > target {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}
