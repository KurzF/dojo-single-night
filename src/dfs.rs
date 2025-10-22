use crate::cities_vec;


pub fn short_route_from(from: &str, map: &Vec<((&str, &str), u32)>) -> u32 {
    let mut to_visit = cities_vec(&map);

    to_visit.retain_mut(|city| *city != from);

    let mut stack: Vec<_> = map.iter()
        .filter(|((from_city, to_city), _dist)| *from_city == from)
        .collect();

    let min = u32::MAX;
    while let Some(((_, to), dist)) = stack.pop() {

    }
    min
}

pub fn short_route(map: &Vec<((&str, &str), u32)>) -> u32 {
    let to_visit = cities_vec(&map);
}
#[cfg(test)]
mod test {
    use crate::{parse_vec, dfs::short_route};


    #[test]
    fn test_route() {
        let input = include_str!("../input.txt");
        let map = parse_vec(input);

        assert_eq!(117, short_route(&map));
    }

}
