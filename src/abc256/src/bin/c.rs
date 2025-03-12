use proconio::input;

fn main() {
    input! {
        h: [isize; 3],
        w: [isize; 3],
    }

    let mut ans = 0;
    for a in 1..=30 {
        for b in 1..=30 {
            for d in 1..=30 {
                for e in 1..=30 {
                    let c = h[0] - a - b;
                    let f = h[1] - d - e;
                    let g = w[0] - a - d;
                    let h_val = w[1] - b - e;
                    let i = w[2] - c - f;
                    let &min = vec![c, f, g, h_val, i].iter().min().unwrap();
                    if min > 0 && g + h_val + i == h[2] {
                        ans += 1;
                    }
                }
            }
        }
    }

    println!("{}", ans)
}
