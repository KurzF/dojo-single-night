use crate::{cities_vec};

pub fn short_route_from(from: &str, to_visit: &mut Vec<&str>, map: &[((&str, &str), u32)]) -> u32 {

    let index = to_visit.iter().position(|c| *c == from).unwrap();
    to_visit.swap_remove(index);
    
    if to_visit.is_empty() {
        return 0;
    }

    // La plus petite distance est la somme du chemin parcourue entre la première et la deuxième
    // vile + la plus petite distance entre les autres villes
    to_visit.iter().map(|city| {
        let dist = map.iter()
            .find(|(edge, _)| *edge == (from, city) || *edge == (city, from))
            .map(|(_, dist)| dist)
            .unwrap();
        dist + short_route_from(city, &mut to_visit.clone(), map)
    }).min()
    .unwrap()
}

pub fn short_route(map: &[((&str, &str), u32)]) -> u32 {
    let to_visit = cities_vec(&map);
    to_visit.iter().map(|city| short_route_from(city, &mut to_visit.clone(), &map)).min().unwrap()
}

#[cfg(test)]
mod test {
    use crate::{parse_vec, recursive::short_route};


    #[test]
    fn test_route() {
        let input = include_str!("../input.txt");
        let map = parse_vec(input);

        assert_eq!(117, short_route(&map));
    }

}
