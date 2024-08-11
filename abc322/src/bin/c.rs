use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }

    for i in 1..=n {
        match a.binary_search(&i) {
            Ok(_) => {
                println!("0");
            }
            Err(index) => {
                if index < m {
                    println!("{}", a[index] - i);
                } else {
                    println!("Error");
                }
            }
        }
    }
}
