use proconio::input;

fn main() {
    input! {
        ax: isize, ay: isize,
        bx: isize, by: isize,
        cx: isize, cy: isize,
        dx: isize, dy: isize,
    }

    let points = [
        Vec2 { x: ax, y: ay },
        Vec2 { x: bx, y: by },
        Vec2 { x: cx, y: cy },
        Vec2 { x: dx, y: dy },
    ];

    if is_convex(&points) {
        println!("Yes");
    } else {
        println!("No");
    }
}

struct Vec2 {
    x: isize,
    y: isize,
}

fn cross_product(v1: Vec2, v2: Vec2) -> isize {
    v1.x * v2.y - v1.y * v2.x
}

fn vector_from_points(p1: &Vec2, p2: &Vec2) -> Vec2 {
    Vec2 {
        x: p2.x - p1.x,
        y: p2.y - p1.y,
    }
}

fn is_convex(points: &[Vec2; 4]) -> bool {
    let mut cross_products = Vec::new();

    for i in 0..4 {
        let current = &points[i];
        let next = &points[(i + 1) % 4];
        let next_next = &points[(i + 2) % 4];

        let v1 = vector_from_points(current, next);
        let v2 = vector_from_points(next, next_next);

        let cross = cross_product(v1, v2);
        cross_products.push(cross);
    }

    cross_products.iter().all(|&cp| cp > 0)
}
