use std::collections::HashMap;

use crate::cities;

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

pub fn short_route(map: &HashMap<(&str, &str), u32>) -> Option<u32> {
    let cities = cities(&map);
    
    cities.iter().flat_map(|city| short_route_from(city, &map)).min()
}

#[cfg(test)]
mod test {
    use crate::{parse_map, recursive::short_route};


    #[test]
    fn test_route() {
        let input = include_str!("../input.txt");
        let map = parse_map(input);

        assert_eq!(Some(117), short_route(&map));
    }

}
