use proconio::input;

fn main() {
    input! {
        txa: f64,
        tya: f64,
        txb: f64,
        tyb: f64,
        t: f64,
        v: f64,
        n: usize,
        xy: [(f64, f64); n],
    }

    let length = t * v;
    let mut answer = false;

    for (x, y) in xy {
        if ((x - txa).powi(2) + (y - tya).powi(2)).sqrt()
            + ((txb - x).powi(2) + (tyb - y).powi(2)).sqrt()
            <= length
        {
            answer = true;
            break;
        }
    }

    println!("{}", if answer { "YES" } else { "NO" })
}
