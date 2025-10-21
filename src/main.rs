use std::collections::{btree_map::Keys, HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");

    let mut map = HashMap::new();

    for line in input.lines() {
        let mut split = line.split_ascii_whitespace();
        let from_city = split.next().unwrap();
        split.next();
        let to_city = split.next().unwrap();
        split.next();
        let value = split.next().unwrap();

        map.insert((from_city, to_city), value.parse::<u32>());
        map.insert((to_city, from_city), value.parse::<u32>());
    }

}

fn short_route(map: HashMap<(&str, &str), u32>) -> u32 {
    let mut cities = HashSet::new();
    
    for key in map.keys() {
        cities.insert(key.0);
    }

    let mut distance_min = u32::MAX;

    for city in cities {

        let map_without_current: HashMap<(&str, &str), u32> = map.iter()
            .filter(|((from, to), _)| *from != city && *to != city)
            .map(|(key, dist)| (*key, *dist))
            .collect();
        
        let destination:Vec<&str> = map.iter()
            .filter(|((from, _), _)| *from == city)
            .map(|(key, _)| key.1)
            .collect();

        
        for d in destination {
            
        }

        //for nextCity in cities 

        if !map_without_current.is_empty() {
            let current_value = short_route(map_without_current);

            if current_value < distance_min {
                distance_min = current_value;
            }
        }
        
    }

    return distance_min;
}

#[cfg(test)]
mod test{
    use std::collections::HashMap;

    use crate::short_route;

    #[test]
    fn test1() {
        let mut map = HashMap::new();

        map.insert(("a", "b"), 10);

        assert_eq!(10, short_route(map));
    }
}