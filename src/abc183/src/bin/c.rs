use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        t: [[usize; n]; n],
    }

    let mut answer = 0;

    for index2 in (1..n).permutations(n - 1) {
        let mut index = vec![0];
        index.extend(index2);
        let mut ti = 0;
        for i in 0..n {
            ti += t[index[i]][index[(i + 1) % n]];
        }
        if ti == k {
            answer += 1;
        }
    }

    println!("{}", answer)
}
