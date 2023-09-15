fn surprise(p: f64) -> f64 {
    assert!((0. ..=1.).contains(&p));
    (1. / p).ln()
}

fn surprise_naive(p: f64) -> f64 {
    assert!((0. ..=1.).contains(&p));
    1. / p - 1.
}

fn main() {
    for val in (0..=10).map(|i| i as f64 * 0.1) {
        println!("s1({}) = {}", val, surprise_naive(val));
        println!("s2({}) = {}", val, surprise(val));
    }
}
