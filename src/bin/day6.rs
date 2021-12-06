use std::collections::HashMap;

fn main() {
    let input : Vec<u32> = advent::read_file(6).trim_end()
                                               .split_terminator(',')
                                               .map(|n| n.parse::<u32>().unwrap())
                                               .collect();
    println!("6a: {}", simulate(&input, 80));
    println!("6b: {}", simulate(&input, 256));
}

fn simulate(input: &[u32], days: u32) -> usize {
    let mut fish = HashMap::new();
    for days in 0 ..= 8 {
        fish.insert(days, 0);
    }
    for f in input {
        let count = fish.entry(*f).or_insert(0);
        *count += 1;
    }

    for _ in 0 .. days {
        let mut new_fish = HashMap::new();
        for remaining in 1 ..= 8 {
            new_fish.insert(remaining - 1, *fish.get(&remaining).unwrap());
        }
        new_fish.insert(8, *fish.get(&0).unwrap());

        let count = new_fish.entry(6).or_insert(0);
        *count += fish.get(&0).unwrap();

        fish = new_fish;
    }

    fish.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [3,4,3,1,2];
        assert_eq!(simulate(&input, 18), 26);
        assert_eq!(simulate(&input, 80), 5934);
    }
}
