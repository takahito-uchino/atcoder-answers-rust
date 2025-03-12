use proconio::input;

fn main() {
    input! {
        l: usize,
    }

    println!("{}", ((l as f64) / 3.).powf(3.))
}
