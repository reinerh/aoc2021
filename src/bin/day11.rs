use std::collections::HashMap;

fn main() {
    let input = advent::read_lines(11);
    println!("11a: {}", count_flashes(&input, 100));
    println!("11b: {}", all_flashing(&input));
}

#[derive(PartialEq,Eq,Hash,Copy,Clone)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Clone)]
struct OctopusCavern {
    grid: HashMap<Position,i8>,
    flashes: usize,
}

impl OctopusCavern {
    fn new<T: AsRef<str>>(input: &[T]) -> OctopusCavern {
        let mut grid = HashMap::new();
        for (y, line) in input.iter().enumerate() {
            for (x, c) in line.as_ref().split_terminator("").skip(1).enumerate() {
                let level = c.parse::<i8>().unwrap();
                grid.insert(Position { x: x as isize, y: y as isize }, level);
            }
        }
        OctopusCavern { grid, flashes: 0 }
    }

    fn increment_levels(&mut self) {
        for (_, level) in self.grid.iter_mut() {
            *level += 1;
        }
    }

    fn can_flash(&self) -> bool {
        self.grid.values().any(|&level| level > 9)
    }

    fn flash(&mut self) {
        let mut new_grid = self.grid.clone();

        for pos in self.grid.iter().filter(|(_,&level)| level > 9).map(|(&pos,_)| pos) {
            self.flashes += 1;
            new_grid.insert(pos, -1);
            for x in -1 ..= 1 {
                for y in -1 ..= 1 {
                    if (x, y) == (0, 0) {
                        continue
                    }

                    if let Some(level) = new_grid.get_mut(&Position { x: pos.x + x, y: pos.y + y }) {
                        if *level != -1 {
                            *level += 1;
                        }
                    }
                }
            }
        }

        self.grid = new_grid;
    }

    fn reset_levels(&mut self) {
        for level in self.grid.values_mut() {
            if *level == -1 {
                *level = 0;
            }
        }
    }

    fn step(&mut self) {
        assert!(!self.can_flash());
        self.increment_levels();
        while self.can_flash() {
            self.flash();
        }
        self.reset_levels();
    }

    fn all_flashing(&self) -> bool {
        !self.grid.values().any(|&level| level > 0)
    }
}


fn count_flashes<T: AsRef<str>>(input: &[T], steps: usize) -> usize {
    let mut cavern = OctopusCavern::new(input);
    for _ in 0 .. steps {
        cavern.step();
    }
    cavern.flashes
}

fn all_flashing<T: AsRef<str>>(input: &[T]) -> usize {
    let mut cavern = OctopusCavern::new(input);
    let mut step = 0;
    loop {
        step += 1;
        cavern.step();
        if cavern.all_flashing() {
            return step;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let input = [
            "11111",
            "19991",
            "19191",
            "19991",
            "11111",
        ];
        assert_eq!(count_flashes(&input, 2), 9);
    }

    #[test]
    fn test1() {
        let input = [
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526",
        ];
        assert_eq!(count_flashes(&input, 100), 1656);
        assert_eq!(all_flashing(&input), 195);
    }

}
