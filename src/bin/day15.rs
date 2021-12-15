use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

fn main() {
    let input = advent::read_lines(15);
    println!("15a: {}", lowest_risk(&input));
    println!("15b: {}", lowest_risk_large(&input));
}

#[derive(PartialEq,Eq,Hash,Clone,Copy,Debug)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(PartialEq,Eq,Clone,Copy)]
struct State {
    pos: Position,
    cost: isize,
}

/* comparator for priority queue */
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/* Dijkstra implementation from https://doc.rust-lang.org/std/collections/binary_heap/index.html */
fn dijkstra(map: &HashMap<Position, isize>, from: &Position, to: &Position) -> isize {
    let mut dist = HashMap::new();
    let mut heap = BinaryHeap::new();

    for pos in map.keys() {
        if pos != from {
            dist.insert(*pos, isize::MAX);
        }
    }

    dist.insert(*from, 0);
    heap.push(State { cost: 0, pos: *from });

    while let Some(State { cost, pos }) = heap.pop() {
        if pos == *to { return cost; }
        if cost > dist[&pos] { continue; }

        for (x, y) in [(1, 0), (0, 1), (0, -1), (-1, 0)] {
            let neigh_pos = Position { x: pos.x + x, y: pos.y + y };
            if !map.contains_key(&neigh_pos) {
                continue;
            }
            let neigh = State { cost: cost + *map.get(&neigh_pos).unwrap(), pos: neigh_pos };
            if neigh.cost < dist[&neigh.pos] {
                heap.push(neigh);
                dist.insert(neigh.pos, neigh.cost);
            }
        }
    }

    panic!("no path found");
}

fn parse_map<T: AsRef<str>>(input: &[T]) -> HashMap<Position, isize> {
    let mut map = HashMap::new();

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.as_ref().chars().enumerate() {
            let risk = c.to_digit(10).unwrap() as isize;
            map.insert(Position { x: x as isize, y: y as isize }, risk);
        }
    }
    map
}

fn lowest_risk<T: AsRef<str>>(input: &[T]) -> isize {
    let map = parse_map(input);

    let start = Position { x: 0, y: 0 };
    let end = *map.keys().max_by_key(|&pos| pos.x + pos.y).unwrap();

    dijkstra(&map, &start, &end)
}

fn lowest_risk_large<T: AsRef<str>>(input: &[T]) -> isize {
    let map = parse_map(input);

    let start = Position { x: 0, y: 0 };
    let end = *map.keys().max_by_key(|&pos| pos.x + pos.y).unwrap();
    let length = end.x + 1;

    let mut large_map = HashMap::new();
    for x in 0 .. 5 {
        for y in 0 .. 5 {
            for (pos, risk) in &map {
                let pos = Position {
                    x: pos.x + x * length,
                    y: pos.y + y * length,
                };
                let risk = ((risk - 1 + x + y) % 9) + 1;
                large_map.insert(pos, risk);
            }
        }
    }
    let end = *large_map.keys().max_by_key(|&pos| pos.x + pos.y).unwrap();

    dijkstra(&large_map, &start, &end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "1163751742",
            "1381373672",
            "2136511328",
            "3694931569",
            "7463417111",
            "1319128137",
            "1359912421",
            "3125421639",
            "1293138521",
            "2311944581",
        ];
        assert_eq!(lowest_risk(&input), 40);
        assert_eq!(lowest_risk_large(&input), 315);
    }
}
