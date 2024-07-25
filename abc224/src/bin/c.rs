use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n],
    }

    let mut answer = 0;

    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if (xy[j].0 - xy[i].0) * (xy[k].1 - xy[i].1)
                    - (xy[k].0 - xy[i].0) * (xy[j].1 - xy[i].1)
                    != 0
                {
                    answer += 1;
                }
            }
        }
    }

    println!("{}", answer)
}
