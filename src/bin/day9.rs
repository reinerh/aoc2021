use std::collections::HashSet;

fn main() {
    let input = advent::read_lines(9);
    println!("9a: {}", risk_levels_low(&input));
    println!("9b: {}", largest_basins(&input));
}

#[derive(Clone,Copy,PartialEq,Eq,Hash)]
struct Position {
    x: usize,
    y: usize,
}

struct Heightmap {
    map: Vec<Vec<u8>>,
}

impl Heightmap {
    fn new<T: AsRef<str>>(input: &[T]) -> Heightmap {
        let map : Vec<Vec<u8>> = input.iter()
                                      .map(|line| line.as_ref().split_terminator("").skip(1).map(|c| c.parse::<u8>().unwrap()).collect())
                                      .collect();
        Heightmap { map }
    }

    fn find_low_points(&self) -> Vec<Position> {
        let mut low_points = Vec::new();
        for y in 0 .. self.map.len() {
            for x in 0 .. self.map[0].len() {
                let current = self.map[y][x];
                if x > 0 && self.map[y][x-1] <= current {
                    continue;
                }
                if x < self.map[y].len() - 1 && self.map[y][x+1] <= current {
                    continue;
                }
                if y > 0 && self.map[y-1][x] <= current {
                    continue;
                }
                if y < self.map.len() - 1 && self.map[y+1][x] <= current {
                    continue;
                }
                low_points.push(Position { x, y });
            }
        }
        low_points
    }

    fn risk_level_at(&self, pos: Position) -> u8 {
        self.map[pos.y][pos.x] + 1
    }

    fn basin_at(&self, pos: Position) -> HashSet<Position> {
        let mut points = HashSet::from([pos]);

        loop {
            let mut new_points = points.clone();
            for pos in &points {
                if pos.x > 0 && self.map[pos.y][pos.x-1] < 9 {
                    new_points.insert(Position { x: pos.x - 1, y: pos.y });
                }
                if pos.x < self.map[pos.y].len() - 1 && self.map[pos.y][pos.x+1] < 9 {
                    new_points.insert(Position { x: pos.x + 1, y: pos.y });
                }
                if pos.y > 0 && self.map[pos.y-1][pos.x] < 9 {
                    new_points.insert(Position { x: pos.x, y: pos.y - 1});
                }
                if pos.y < self.map.len() - 1 && self.map[pos.y+1][pos.x] < 9 {
                    new_points.insert(Position { x: pos.x, y: pos.y + 1});
                }
            }
            if points.len() == new_points.len() {
                break;
            }
            points = new_points;
        }
        points
    }
}

fn risk_levels_low<T: AsRef<str>>(input: &[T]) -> usize {
    let heightmap = Heightmap::new(input);

    heightmap.find_low_points()
             .iter()
             .map(|&pos| heightmap.risk_level_at(pos) as usize)
             .sum()
}

fn largest_basins<T: AsRef<str>>(input: &[T]) -> usize {
    let heightmap = Heightmap::new(input);
    let lowpoints = heightmap.find_low_points();

    let mut basin_sizes : Vec<usize> = lowpoints.iter()
                                                .map(|&pos| heightmap.basin_at(pos))
                                                .map(|basin| basin.len())
                                                .collect();
    basin_sizes.sort_unstable();
    basin_sizes.iter()
               .rev()
               .take(3)
               .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ];
        assert_eq!(risk_levels_low(&input), 15);
        assert_eq!(largest_basins(&input), 1134);
    }
}
