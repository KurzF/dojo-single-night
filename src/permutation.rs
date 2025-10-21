// Heap's algorithm
fn permutation<T, F>(elements: &mut [T], mut callback: F)
where F: FnMut(&[T])
{
	let mut c = vec![0; elements.len()];

	callback(elements);

	let mut i = 1;
	while i < elements.len() {
		if c[i] < i {
			if i % 2 == 0 {
				elements.swap(0, i);
			} else {
				elements.swap(c[i], i);
			}
			callback(elements);
			c[i] += 1;
			i = 1
		} else {
			c[i] = 0;
			i += 1;
		}
	}
}

fn cities<'a>(map: &'a Vec<((&'a str, &'a str), u32)>) -> Vec<&'a str> {
    let mut to_visit = Vec::new();
    for ((node1, node2), _) in  map.iter() {
        if !to_visit.contains(node1) {
            to_visit.push(node1);
        }
        if !to_visit.contains(node2) {
            to_visit.push(node2);
        }
    }

    to_visit
}

fn get_distance(map: &Vec<((&str, &str), u32)>, node1: &str, node2: &str) -> u32 {
    map.iter().find(|((n1, n2), _)| (*n1 == node1 && *n2 == node2) || (*n1 == node2 && *n2 == node1)).unwrap().1
} 



pub fn short_route(map: &Vec<((&str, &str), u32)>) -> u32 {
    let mut to_visit = cities(&map);
    let mut min_dist = u32::MAX;
    permutation(&mut to_visit, |permutation| {
        let dist = permutation.windows(2).map(|e| {
			get_distance(&map, e[0], e[1])
		}).sum();

		min_dist = std::cmp::min(min_dist, dist);

    });

    min_dist
}
#[cfg(test)]
mod test {
    use crate::{parse_vec, permutation::short_route};


    #[test]
    fn test_route() {
        let input = include_str!("../input.txt");
        let map = parse_vec(input);

        assert_eq!(117, short_route(&map));
    }

}
