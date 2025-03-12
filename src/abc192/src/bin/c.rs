use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut a = n;

    for _ in 0..k {
        let mut g_str = a.to_string().chars().collect::<Vec<char>>();
        g_str.sort();

        let g2: usize = g_str.iter().collect::<String>().parse().unwrap();

        g_str.reverse();
        let g1: usize = g_str.iter().collect::<String>().parse().unwrap();

        a = g1 - g2;
    }

    println!("{}", a)
}
