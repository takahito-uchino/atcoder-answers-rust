use proconio::input;

fn main() {
    input! {
        x: isize,
    }

    let mut res = vec![0isize];

    for d in 0..60 {
        if x & (1 << d) != 0 {
            let sz = res.len();
            for i in 0..sz {
                res.push(res[i] | (1 << d));
            }
        }
    }

    res.sort();

    for n in res {
        println!("{}", n)
    }
}

