use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    }

    let total = a.iter().sum::<usize>();

    if total <= m {
        println!("infinite");
        return;
    }

    a.sort_unstable();

    let mut ok = 0;
    let mut ng = 1000000000;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        let mut tmp = 0;
        for &i in &a {
            tmp += mid.min(i);
        }
        if tmp <= m {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok)
}
