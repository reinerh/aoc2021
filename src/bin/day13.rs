use regex::Regex;
use std::collections::HashSet;

fn main() {
    let input = advent::read_lines(13);
    println!("13a: {}", fold_once(&input));
    println!("13b:");
    fold_all(&input);
}

#[derive(PartialEq,Eq,Hash,Clone,Copy)]
struct Position {
    x: usize,
    y: usize,
}

struct Fold {
    pos: Position,
}

fn parse_instructions<T: AsRef<str>>(input: &[T]) -> (HashSet<Position>, Vec<Fold>) {
    let re_pos = Regex::new(r"^(\d+),(\d+)$").unwrap();
    let re_fold = Regex::new(r"^fold along ([xy])=(\d+)$").unwrap();
    let mut positions = HashSet::new();
    let mut folds = Vec::new();

    for line in input {
        if let Some(pos) = re_pos.captures(line.as_ref()) {
            positions.insert(Position {
                x: pos.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                y: pos.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            });
            continue;
        }
        if let Some(fold) = re_fold.captures(line.as_ref()) {
            let axis = fold.get(1).unwrap().as_str();
            let coord = fold.get(2).unwrap().as_str().parse::<usize>().unwrap();
            match axis {
                "x" => folds.push(Fold { pos: Position { x: coord, y: 0 } }),
                "y" => folds.push(Fold { pos: Position { x: 0, y: coord } }),
                _ => panic!("invalid axis"),
            }
        }
    }
    (positions, folds)
}

fn fold(points: &HashSet<Position>, fold: &Fold) -> HashSet<Position> {
    let mut new_points = HashSet::new();

    for point in points {
        if fold.pos.x == 0 {
            if point.y < fold.pos.y {
                new_points.insert(*point);
            } else {
                new_points.insert(Position { x: point.x, y: fold.pos.y - (point.y - fold.pos.y) });
            }
        } else if fold.pos.y == 0 {
            if point.x < fold.pos.x {
                new_points.insert(*point);
            } else {
                new_points.insert(Position { x: fold.pos.x - (point.x - fold.pos.x), y: point.y });
            }
        } else {
            panic!("invalid fold");
        }
    }

    new_points
}

fn fold_once<T: AsRef<str>>(input: &[T]) -> usize {
    let (positions, folds) = parse_instructions(input);

    fold(&positions, &folds[0]).iter().len()
}

fn fold_all<T: AsRef<str>>(input: &[T]) {
    let (mut positions, folds) = parse_instructions(input);

    for f in folds {
        positions = fold(&positions, &f);
    }
    let max_x = positions.iter().map(|&pos| pos.x).max().unwrap();
    let max_y = positions.iter().map(|&pos| pos.y).max().unwrap();

    for y in 0 ..= max_y {
        for x in 0 ..= max_x {
            if positions.contains(&Position { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "6,10",
            "0,14",
            "9,10",
            "0,3",
            "10,4",
            "4,11",
            "6,0",
            "6,12",
            "4,1",
            "0,13",
            "10,12",
            "3,4",
            "3,0",
            "8,4",
            "1,10",
            "2,14",
            "8,10",
            "9,0",
            "",
            "fold along y=7",
            "fold along x=5",
        ];
        assert_eq!(fold_once(&input), 17);
    }
}
