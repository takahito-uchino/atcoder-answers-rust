use proconio::input;

fn main() {
    input! {
        xa: f64,
        ya: f64,
        xb: f64,
        yb: f64,
        xc: f64,
        yc: f64,
    }

    let a = xb - xa;
    let b = yb - ya;
    let c = xc - xa;
    let d = yc - ya;

    let answer = (a * d - b * c).abs() / 2.;

    println!("{}", answer)
}
