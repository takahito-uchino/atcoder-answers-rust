use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut c = Vec::new();

    c.extend(a.clone());
    c.extend(b.clone());

    c.sort_unstable();

    for i in 0..n + m - 1 {
        if a.contains(&c[i]) && a.contains(&c[i + 1]) {
            println!("Yes");
            return;
        }
    }

    println!("No")
}
