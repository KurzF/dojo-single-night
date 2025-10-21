use std::{collections::{HashMap, HashSet}, u32};

fn main() {
    let input = include_str!("../input.txt");

    let map = parse(&input);
    let result = short_route(&map);

    println!("Shortest distance: {:?}", result);
}

fn parse(input: &str) -> HashMap<(&str, &str), u32> {
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

/// Extrait toutes les villes présentent dans la carte
fn cities<'a>(map: &HashMap<(&'a str, &'a str), u32>) -> HashSet<&'a str> {
    map.keys().map(|(from, _to)| *from).collect()
}

fn short_route_from(from: &str, map: &HashMap<(&str, &str), u32>) -> Option<u32> {
    // Liste des villes a visiter
    let mut to_visit = cities(&map);
    to_visit.remove(from);

    // Aucune ville restante
    if to_visit.is_empty() {
        return Some(0);
    }

    // Carte sans la ville actuelle
    let map_without_current: HashMap<(&str, &str), u32> = map.iter()
        .filter(|((from_city, to_city), _)| *from_city != from && *to_city != from)
        .map(|(key, dist)| (*key, *dist))
        .collect();

    // La plus petite distance est la somme du chemin parcourue entre la première et la deuxième
    // vile + la plus petite distance entre les autres ville
    to_visit.iter().flat_map(|city| {
        map.get(&(from, city))
            .and_then(|d| short_route_from(city, &map_without_current)
            .map(|sr| sr + d))
    }).min()
}

fn short_route(map: &HashMap<(&str, &str), u32>) -> Option<u32> {
    let cities = cities(&map);
    
    cities.iter().flat_map(|city| short_route_from(city, &map)).min()
}

#[cfg(test)]
mod test{
    use crate::*;

    #[test]
    fn test_parse() {
        let input = "Faerun to Tristram = 65";
        let from = "Faerun";
        let to = "Tristram";

        let result = parse(input);

        assert_eq!(Some(65), result.get(&(from, to)).copied());
        assert_eq!(Some(65), result.get(&(to, from)).copied());
    }

    #[test]
    fn test_cities() {
        let input = "a to b = 10";
        let map = parse(input);

        let result = cities(&map);
        let expected = ["a", "b"].iter().copied().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test1() {
        let input = "a to b = 10";
        let map = parse(input);

        assert_eq!(Some(10), short_route(&map));
    }
}
