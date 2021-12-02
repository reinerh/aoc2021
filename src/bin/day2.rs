struct Position {
    x: i32,
    y: i32,
}

struct AimedPosition {
    pos: Position,
    aim: i32,
}

fn main() {
    let input = advent::read_lines(2);

    let pos = drive_course(Position {x: 0, y: 0}, &input);
    println!("2a: {}", pos.x * pos.y);

    let pos = drive_course_aimed(Position {x: 0, y: 0}, &input);
    println!("2b: {}", pos.x * pos.y);
}

fn move_submarine(pos: Position, cmd: &str) -> Position {
    let tokens : Vec<&str> = cmd.split(' ').collect();
    let direction = tokens[0];
    let distance = tokens[1].parse::<i32>().unwrap();

    let mut pos = pos;
    match direction {
        "forward" => {
            pos.x += distance;
        },
        "down" => {
            pos.y += distance;
        },
        "up" => {
            pos.y -= distance;
        },
        _ => panic!("invalid direction: {}", direction)
    };

    pos
}

fn move_submarine_aimed(pos: AimedPosition, cmd: &str) -> AimedPosition {
    let tokens : Vec<&str> = cmd.split(' ').collect();
    let direction = tokens[0];
    let distance = tokens[1].parse::<i32>().unwrap();

    let mut pos = pos;
    match direction {
        "forward" => {
            pos.pos.x += distance;
            pos.pos.y += pos.aim * distance;
        },
        "down" => {
            pos.aim += distance;
        },
        "up" => {
            pos.aim -= distance;
        },
        _ => panic!("invalid direction: {}", direction)
    };

    pos
}

fn drive_course<T: AsRef<str>>(start: Position, course: &[T]) -> Position {
    let mut pos = start;
    for cmd in course {
        pos = move_submarine(pos, cmd.as_ref());
    }
    pos
}

fn drive_course_aimed<T: AsRef<str>>(start: Position, course: &[T]) -> Position {
    let mut pos = AimedPosition { pos: start, aim: 0 };
    for cmd in course {
        pos = move_submarine_aimed(pos, cmd.as_ref());
    }
    pos.pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let course = [
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        let pos = drive_course(Position {x: 0, y: 0}, &course);
        assert_eq!(pos.x * pos.y, 150);

        let pos = drive_course_aimed(Position {x: 0, y: 0}, &course);
        assert_eq!(pos.x * pos.y, 900);
    }
}
