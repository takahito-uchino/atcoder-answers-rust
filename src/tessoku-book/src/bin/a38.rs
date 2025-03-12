use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        lrh: [(usize, usize, usize); n],
    }

    let mut hours = vec![24; d];

    for (l, r, h) in lrh {
        for i in (l - 1)..r {
            hours[i] = hours[i].min(h);
        }
    }

    let answer = hours.iter().sum::<usize>();

    println!("{}", answer);
}
