use proconio::input;

fn main() {
    input! {
       n: usize,
    }

    let mut s = Vec::new();
    let mut c = Vec::new();

    for _ in 0..n {
        input! {
            si: String,
            ci: usize,
        }

        s.push(si);
        c.push(ci);
    }

    let sum = c.iter().sum::<usize>();
    s.sort_unstable();

    println!("{}", s[sum % n])
}
