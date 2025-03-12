use proconio::input;

fn main() {
    input! {
        n: usize,
        x: isize,
        mut a: [isize; n],
    }

    a.sort_unstable();
    for j in 0..n {
        match a.binary_search(&(a[j] + x)) {
            Ok(_) => {
                println!("Yes");
                return;
            }
            Err(_) => {}
        }
    }

    println!("No")
}
