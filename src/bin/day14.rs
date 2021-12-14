use std::collections::HashMap;

fn main() {
    let input = advent::read_lines(14);
    println!("14a: {}", result_after(&input, 10));
    println!("14b: {}", result_after(&input, 40));
}

struct Polymerizer {
    template: String,
    rules: HashMap<(char,char), char>,
    pairs: HashMap<(char,char), usize>,
}

impl Polymerizer {
    fn new<T: AsRef<str>>(input: &[T]) -> Polymerizer {
        let template = String::from(input[0].as_ref());

        let mut rules = HashMap::new();
        for line in input.iter().skip(2) {
            let (left, right) = line.as_ref().split_once(" -> ").unwrap();
            let mut rule_chars = left.chars();

            let right = right.chars().next().unwrap();
            let left = (rule_chars.next().unwrap(), rule_chars.next().unwrap());
            rules.insert(left, right);
        }

        let mut pairs = HashMap::new();
        for (i, c) in template.chars().enumerate().skip(1) {
            let mut pair = template.chars().skip(i-1);
            let pair = (pair.next().unwrap(), c);
            let entry = pairs.entry(pair).or_insert(0);
            *entry += 1;
        }

        Polymerizer { template, rules, pairs }
    }

    fn step(&mut self) {
        let mut new_pairs = HashMap::new();

        for (pair, count) in &self.pairs {
            let out = *self.rules.get(pair).unwrap();
            let pair1 = (pair.0, out);
            let pair2 = (out, pair.1);

            let count1 = new_pairs.entry(pair1).or_insert(0);
            *count1 += count;
            let count2 = new_pairs.entry(pair2).or_insert(0);
            *count2 += count;
        }

        self.pairs = new_pairs;
    }

    fn score(&self) -> usize {
        let mut chars = HashMap::new();
        chars.insert(self.template.chars().last().unwrap(), 1);

        /* count first element of every pair */
        for (pair, pair_count) in &self.pairs {
            let char_count = chars.entry(pair.0).or_insert(0);
            *char_count += pair_count;
        }
        let max = chars.values().max().unwrap();
        let min = chars.values().min().unwrap();
        max - min
    }
}

fn result_after<T:AsRef<str>>(input: &[T], n: usize) -> usize {
    let mut polymerizer = Polymerizer::new(input);
    for _ in 0 .. n {
        polymerizer.step();
    }
    polymerizer.score()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "NNCB",
            "",
            "CH -> B",
            "HH -> N",
            "CB -> H",
            "NH -> C",
            "HB -> C",
            "HC -> B",
            "HN -> C",
            "NN -> C",
            "BH -> H",
            "NC -> B",
            "NB -> B",
            "BN -> B",
            "BB -> N",
            "BC -> B",
            "CC -> N",
            "CN -> C",
        ];
        assert_eq!(result_after(&input, 10), 1588);
        assert_eq!(result_after(&input, 40), 2188189693529);
    }
}
