#[derive(Debug, Clone)]
struct CompleteGragh<N, W> {
    nodes: Vec<N>,
    weights: Vec<W>
}

impl<N, W> CompleteGragh<N, W> 
where N: Eq + Copy,
      W: Default + Copy
{
    pub fn new(edges: &[((N, N), W)]) -> Self {
        let mut nodes = Vec::new();
		for ((node1, node2), _) in  edges.iter() {
			if !nodes.contains(node1) {
				nodes.push(*node1);
			}
			if !nodes.contains(node2) {
				nodes.push(*node2);
			}
		}
        
        let mut weights = vec![W::default(); nodes.len() * nodes.len()];

		for ((node1, node2), weight) in edges.iter() {
            let i1 = nodes.iter().position(|n| n == node1).unwrap();
            let i2 = nodes.iter().position(|n| n == node2).unwrap();

            weights[i1 + i2 * nodes.len()] = *weight;
            weights[i2 + i1 * nodes.len()] = *weight;
        }

        Self {
            nodes,
            weights
        }
    }
}

impl<N> CompleteGragh<N, u32> 
where N: Eq + Copy 
{
    fn short_route_from(&self, from: usize, mut visited: Visited, memo: &mut Vec<((usize, Visited), u32)>) -> u32 {
        visited.visit(from);

        // Everything is visited
        if visited.count() as usize == self.nodes.len() {
            return 0;
        }

        let precalculated = memo.iter()
                .find(|((f, v), _)| *f == from && *v == visited)
                .map(|(_, dist)| *dist);

        match precalculated {
            Some(p) => return p,
            None => {},
        }

        let minimum = (0..self.nodes.len())
            .filter(|i| !visited.is_visited(*i))
            .map(|next| {
                let dist = self.weights[from + next * self.nodes.len()];
                dist + self.short_route_from(next, visited, memo)
            }).min().unwrap();

        memo.push(((from, visited), minimum));

        minimum
    }
    

    fn short_route(&self) -> u32 {
        let mut memo = Vec::new();
        self.nodes.iter().enumerate().map(|(i, _city)| self.short_route_from(i, Visited::empty(), &mut memo)).min().unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Visited(u32);

impl Visited {
    pub fn empty() -> Self {
        Self(0)
    }

    fn visit(&mut self, i: usize) {
        if i >= 32 {
            panic!("Support 32 node max");
        }
        self.0 |= 1 << i
    }

    fn is_visited(&self, i: usize) -> bool {
        self.0 & 1 << i != 0
    }

    fn count(&self) -> u32 {
        self.0.count_ones()
    }
}

pub fn short_route(map: &[((&str, &str), u32)]) -> u32 {
    let graph = CompleteGragh::new(map);
    graph.short_route()
}

#[cfg(test)]
mod test {
    use crate::{parse_vec, dp::short_route};


    #[test]
    fn test_route() {
        let input = include_str!("../input.txt");
        let map = parse_vec(input);

        assert_eq!(117, short_route(&map));
    }

}
