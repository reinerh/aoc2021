fn main() {
    let (x1, x2) = (269, 292);
    let (y1, y2) = (-68, -44);
    println!("17a: {}", highest_y(x1, x2, y1, y2));
    println!("17b: {}", count_velocities(x1, x2, y1, y2));
}

fn hit_target_y(velocity: i64, y1: i64, y2: i64, steps: i64) -> bool {
    let mut pos = 0;
    let mut velocity = velocity;

    /* steps already done until 0 is reached again */
    let initial_steps = velocity * 2 + 1;

    for _ in initial_steps .. steps {
        velocity += 1;
        pos -= velocity;
    }

    pos >= y1 && pos <= y2
}

fn hit_target_x(velocity: i64, x1: i64, x2: i64, steps: i64) -> bool {
    let mut pos = 0;
    let mut velocity = velocity;

    for _ in 0 .. steps {
        pos += velocity;
        velocity -= 1;
        velocity = std::cmp::max(0, velocity);
    }

    pos >= x1 && pos <= x2
}

fn peak_y(velocity: i64) -> i64 {
    velocity * (velocity + 1) / 2
}

fn highest_y(x1: i64, x2: i64, y1: i64, y2: i64) -> i64 {
    let mut max = 0;
    for vel_x in 1 .. 1000 {
        for vel_y in 0 .. 1000 {
            for steps in 1 .. 150 {
                if hit_target_x(vel_x, x1, x2, steps) && hit_target_y(vel_y, y1, y2, steps) {
                    max = std::cmp::max(max, peak_y(vel_y));
                    break;
                }
            }
        }
    }
    max
}

fn count_velocities(x1: i64, x2: i64, y1: i64, y2: i64) -> i64 {
    let mut count = 0;
    for vel_x in 1 .. 1000 {
        for vel_y in -1000 .. 1000 {
            for steps in 1 .. 150 {
                if hit_target_x(vel_x, x1, x2, steps) && hit_target_y(vel_y, y1, y2, steps) {
                    count += 1;
                    break;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let (x1, x2) = (20, 30);
        let (y1, y2) = (-10, -5);

        assert!(!hit_target_x(7, x1, x2, 3));
        assert!(hit_target_x(7, x1, x2, 4));
        assert!(hit_target_x(7, x1, x2, 5));
        assert!(hit_target_x(7, x1, x2, 6));
        assert!(hit_target_x(7, x1, x2, 7));

        assert!(!hit_target_y(2, y1, y2, 6));
        assert!(hit_target_y(2, y1, y2, 7));
        assert!(!hit_target_y(2, y1, y2, 8));

        assert!(!hit_target_x(6, x1, x2, 4));
        assert!(hit_target_x(6, x1, x2, 5));
        assert!(hit_target_x(6, x1, x2, 6));
        assert!(hit_target_x(6, x1, x2, 7));
        assert!(hit_target_x(6, x1, x2, 8));
        assert!(hit_target_x(6, x1, x2, 9));
        assert!(hit_target_x(6, x1, x2, 10));

        assert!(!hit_target_y(3, y1, y2, 8));
        assert!(hit_target_y(3, y1, y2, 9));
        assert!(!hit_target_y(3, y1, y2, 10));

        assert_eq!(highest_y(x1, x2, y1, y2), 45);
        assert_eq!(count_velocities(x1, x2, y1, y2), 112);
    }
}
