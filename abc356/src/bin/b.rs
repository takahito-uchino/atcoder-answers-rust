use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
        x: [[usize; m]; n],
    }

    for i in 0..m {
        let mut sum = 0;

        for j in 0..n {
            sum += x[j][i];
        }

        if sum < a[i] {
            println!("No");
            return;
        }
    }

    println!("Yes")
}
