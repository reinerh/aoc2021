fn main() {
    let input : Vec<i32> = advent::read_file(7).trim_end()
                                               .split_terminator(',')
                                               .map(|n| n.parse::<i32>().unwrap())
                                               .collect();
    println!("7a: {}", least_fuel(&input));
    println!("7b: {}", least_crab_fuel(&input));
}

fn least_fuel(positions: &[i32]) -> i32 {
    let min_pos = *positions.iter().min().unwrap();
    let max_pos = *positions.iter().max().unwrap();

    (min_pos ..= max_pos).map(|pos| fuel_at_pos(positions, pos))
                         .min()
                         .unwrap()
}

fn fuel_at_pos(positions: &[i32], dest: i32) -> i32 {
    positions.iter()
             .map(|pos| (pos - dest).abs())
             .sum()
}

fn least_crab_fuel(positions: &[i32]) -> i32 {
    let min_pos = *positions.iter().min().unwrap();
    let max_pos = *positions.iter().max().unwrap();

    (min_pos ..= max_pos).map(|pos| crab_fuel_at_pos(positions, pos))
                         .min()
                         .unwrap()
}

fn gauss_sum(n: i32) -> i32 {
    n * (n + 1) / 2
}

fn crab_fuel_at_pos(positions: &[i32], dest: i32) -> i32 {
    positions.iter()
             .map(|pos| gauss_sum((pos - dest).abs()))
             .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let positions = [16,1,2,0,4,2,7,1,2,14];
        assert_eq!(fuel_at_pos(&positions, 2), 37);
        assert_eq!(fuel_at_pos(&positions, 1), 41);
        assert_eq!(fuel_at_pos(&positions, 3), 39);
        assert_eq!(fuel_at_pos(&positions, 10), 71);
        assert_eq!(least_fuel(&positions), 37);

        assert_eq!(crab_fuel_at_pos(&positions, 2), 206);
        assert_eq!(least_crab_fuel(&positions), 168);
    }
}
