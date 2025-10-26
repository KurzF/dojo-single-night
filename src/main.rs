use dojo_11::{parse_vec, recursive::short_route};


fn main() {
    let input = include_str!("../input.txt");

    let map = parse_vec(&input);
    let result = short_route(&map);

    println!("Shortest distance: {:?}", result);
}

