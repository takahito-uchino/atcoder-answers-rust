use proconio::input;

fn main() {
    input! {
        xa: isize,
        ya: isize,
        xb: isize,
        yb: isize,
        xc: isize,
        yc: isize,
    }

    let ab = (xa - xb).pow(2) + (ya - yb).pow(2);
    let bc = (xb - xc).pow(2) + (yb - yc).pow(2);
    let ca = (xc - xa).pow(2) + (yc - ya).pow(2);

    if (ab + bc == ca) || (bc + ca == ab) || (ca + ab == bc) {
        println!("Yes");
    } else {
        println!("No");
    }
}
