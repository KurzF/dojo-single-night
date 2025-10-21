use std::{collections::{HashMap, HashSet}, u32};

pub mod recursive;
pub mod permutation;

pub fn parse_map(input: &str) -> HashMap<(&str, &str), u32> {
    let mut map = HashMap::new();

    for line in input.lines() {
        let mut split = line.split_ascii_whitespace();
        let from_city = split.next().unwrap();
        split.next();
        let to_city = split.next().unwrap();
        split.next();
        let value: u32 = split
            .next().unwrap()
            .parse().unwrap();

        map.insert((from_city, to_city), value);
        map.insert((to_city, from_city), value);
    }

    map
}

pub fn parse_vec(input: &str) -> Vec<((&str, &str), u32)> {
    input.lines().map(|l| {
        let mut split = l.split_ascii_whitespace();
        let from_city = split.next().unwrap();
        split.next();
        let to_city = split.next().unwrap();
        split.next();
        let value: u32 = split
            .next().unwrap()
            .parse().unwrap();

        ((from_city, to_city), value)
    }).collect()
}


/// Extrait toutes les villes présentent dans la carte
pub(crate) fn cities<'a>(map: &HashMap<(&'a str, &'a str), u32>) -> HashSet<&'a str> {
    map.keys().map(|(from, _to)| *from).collect()
}


#[cfg(test)]
mod test{
    use crate::*;

    #[test]
    fn test_parse() {
        let input = "Faerun to Tristram = 65";
        let from = "Faerun";
        let to = "Tristram";

        let result = parse_map(input);

        assert_eq!(Some(65), result.get(&(from, to)).copied());
        assert_eq!(Some(65), result.get(&(to, from)).copied());
    }

    #[test]
    fn test_cities() {
        let input = "a to b = 10";
        let map = parse_map(input);

        let result = cities(&map);
        let expected = ["a", "b"].iter().copied().collect();
        assert_eq!(result, expected);
    }
}
