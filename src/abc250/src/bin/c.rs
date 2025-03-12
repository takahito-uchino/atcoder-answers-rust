use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        x: [usize; q],
    }
    let mut val = vec![0; n + 1];
    let mut pos = vec![0; n + 1];
    for i in 1..=n {
        val[i] = i;
        pos[i] = i;
    }

    for i in 0..q {
        let p0 = pos[x[i]];
        let mut p1 = p0;
        if p0 != n {
            p1 += 1;
        } else {
            p1 -= 1;
        }
        let v0 = val[p0];
        let v1 = val[p1];
        val.swap(p0, p1);
        pos.swap(v0, v1);
    }

    println!("{}", val[1..=n].iter().map(|num| num.to_string()).join(" "));
}
