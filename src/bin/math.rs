/// Returns the information content in an event with probability p
fn surprisal(p: f64) -> f64 {
    assert!((0. ..=1.).contains(&p));
    - p.log2()
}

fn odds(p: f64) -> f64 {
    assert!((0. ..=1.).contains(&p));
    p / (1. - p)
}

fn main() {
    println!("s : p \u{2192} \u{211D}+");
    for val in (0..=10).map(|i| i as f64 * 0.1) {
        println!("sup({:.1}) = {:.3}", val, surprisal(val));
        println!("odd({:.1}) = {:.3}", val, odds(val));
    }
}
