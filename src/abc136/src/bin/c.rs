use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    for i in (0..n - 1).rev() {
        if a[i] > a[i + 1] + 1 {
            println!("No");
            return;
        } else if a[i] == a[i + 1] + 1 {
            a[i] -= 1;
        }
    }

    println!("Yes")
}
