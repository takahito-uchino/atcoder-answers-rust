use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut a = Vec::new();

    for i in 0..n {
        a.push(i + 1);
    }

    let mut reverse = false;

    for _ in 0..q {
        input! {
            t: usize,
        }

        if t == 1 {
            input! {
                x: usize,
                y: usize,
            }

            if !reverse {
                a[x - 1] = y;
            } else {
                a[n - x] = y;
            }
        }

        if t == 2 {
            reverse = if !reverse { true } else { false };
        }

        if t == 3 {
            input! {
                x: usize
            }

            if !reverse {
                println!("{}", a[x - 1])
            } else {
                println!("{}", a[n - x])
            }
        }
    }
}
