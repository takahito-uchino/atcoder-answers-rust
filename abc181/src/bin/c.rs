use proconio::input;

fn main() {
    input! {
        n: usize,
        points: [(isize, isize); n],
    }

    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            for k in j + 1..n {
                let x1 = points[j].0 - points[i].0;
                let x2 = points[k].0 - points[i].0;
                let y1 = points[j].1 - points[i].1;
                let y2 = points[k].1 - points[i].1;

                if x2 * y1 == x1 * y2 {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No")
}
