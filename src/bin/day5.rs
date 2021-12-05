use std::collections::HashMap;

fn main() {
    let input = advent::read_lines(5);
    println!("5a: {}", count_overlaps(&input, false));
    println!("5b: {}", count_overlaps(&input, true));
}

#[derive(PartialEq,Eq,Hash)]
struct Point {
    x: isize,
    y: isize,
}

struct Line {
    start: Point,
    end: Point,
}

impl Point {
    fn new(coord: &str) -> Point {
        let (x_str, y_str) = coord.split_once(',').unwrap();
        let x = x_str.parse::<isize>().unwrap();
        let y = y_str.parse::<isize>().unwrap();
        Point { x, y }
    }
}

impl Line {
    fn new(points: &str) -> Line {
        let (start_str, end_str) = points.split_once(" -> ").unwrap();
        let (start, end) = (Point::new(start_str), Point::new(end_str));
        Line { start, end }
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn points_on_line(&self) -> Vec<Point> {
        let mut points = Vec::new();

        let x_diff = self.end.x - self.start.x;
        let y_diff = self.end.y - self.start.y;

        if x_diff == 0 {
            /* vertical line */
            for y in 0 ..= y_diff.abs() {
                points.push(Point { x: self.start.x, y: self.start.y + y_diff.signum() * y });
            }
        } else if y_diff == 0 {
            /* horizontal line */
            for x in 0 ..= x_diff.abs() {
                points.push(Point { x: self.start.x + x_diff.signum() * x, y: self.start.y });
            }
        } else if x_diff.abs() == y_diff.abs() && x_diff.abs() > 0 {
            /* diagonal line */
            for i in 0 ..= x_diff.abs() {
                let x = self.start.x + x_diff.signum() * i;
                let y = self.start.y + y_diff.signum() * i;
                points.push(Point { x, y });
            }
        } else {
            panic!("No line!");
        }

        points
    }
}

fn count_overlaps<T: AsRef<str>>(input: &[T], with_diagonal: bool) -> usize {
    let lines : Vec<Line> = input.iter()
                                 .map(|line| Line::new(line.as_ref()))
                                 .collect();

    let mut map = HashMap::new();
    for line in lines {
        if !with_diagonal && !line.is_horizontal() && !line.is_vertical() {
            continue;
        }
        for point in line.points_on_line() {
            let count = map.entry(point).or_insert(0);
            *count += 1;
        }
    }
    map.values()
       .filter(|&count| *count > 1)
       .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ];
        assert_eq!(count_overlaps(&input, false), 5);
        assert_eq!(count_overlaps(&input, true), 12);
    }
}
