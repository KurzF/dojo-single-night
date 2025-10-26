use dojo_11::{dp, parse_vec, permutation, recursive};

fn main() {
    divan::main()
}

const INPUT: &str = include_str!("../input.txt");

#[divan::bench]
fn recursive(bencher: divan::Bencher) {
    let map = parse_vec(INPUT);

    bencher.bench(|| {
        recursive::short_route(&map);
    });
}

#[divan::bench]
fn permutation(bencher: divan::Bencher) {
    let map = parse_vec(INPUT);

    bencher.bench(|| {
        permutation::short_route(&map);
    });
}

#[divan::bench]
fn dp(bencher: divan::Bencher) {
    let map = parse_vec(INPUT);

    bencher.bench(|| {
        dp::short_route(&map);
    });
}
