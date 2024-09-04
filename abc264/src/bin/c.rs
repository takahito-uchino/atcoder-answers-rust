use proconio::input;

fn main() {
    input! {
        h1: usize,
        w1: usize,
        a: [[usize; w1]; h1],
        h2: usize,
        w2: usize,
        b: [[usize; w2]; h2],
    }

    for i in 0..(1 << h1) {
        for j in 0..(1 << w1) {
            let mut h_vec = Vec::new();
            let mut w_vec = Vec::new();

            for k in 0..h1 {
                if i & (1 << k) == 0 {
                    h_vec.push(k);
                }
            }

            for k in 0..w1 {
                if j & (1 << k) == 0 {
                    w_vec.push(k);
                }
            }

            if h_vec.len() != h2 || w_vec.len() != w2 {
                continue;
            }

            let mut match_found = true;

            for k in 0..h2 {
                for l in 0..w2 {
                    if a[h_vec[k]][w_vec[l]] != b[k][l] {
                        match_found = false;
                        break;
                    }
                }
                if !match_found {
                    break;
                }
            }

            if match_found {
                println!("Yes");
                return;
            }
        }
    }

    println!("No")
}
