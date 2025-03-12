use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0;

    for i in 1..n {
        let x = i;
        let y = n - i;
        let mut a = 0;
        let mut b = 0;

        let mut j = 1;
        while j * j <= x {
            if x % j == 0 {
                a += 1;
                if x != j * j {
                    a += 1;
                }
            }
            j += 1;
        }

        let mut k = 1;
        while k * k <= y {
            if y % k == 0 {
                b += 1;
                if y != k * k {
                    b += 1;
                }
            }
            k += 1;
        }
        ans += a * b;
    }

    println!("{}", ans)
}
