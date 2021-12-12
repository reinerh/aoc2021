use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = advent::read_lines(12);
    println!("12a: {}", paths_through_caves(&input, false));
    println!("12b: {}", paths_through_caves(&input, true));
}

fn is_small_cave(cave: &str) -> bool {
    !cave.chars().any(|c| c.is_uppercase())
}

fn visiting_allowed(visited: &HashMap<&str, usize>, cave: &str, allow_twice: bool) -> bool {
    if cave == "start" {
        return false;
    }

    if !is_small_cave(cave) {
        return true;
    }

    if !allow_twice {
        return !visited.contains_key(cave);
    }

    match visited.get(cave) {
        None => true,
        Some(count) => {
            assert!(*count > 0);
            !visited.iter().any(|(cave,c)| is_small_cave(cave) && *c == 2)
        }
    }
}

fn count_paths<'a>(connections: &HashMap<&str, HashSet<&'a str>>, visited: &mut HashMap<&'a str, usize>, from: &'a str, to: &str, allow_twice: bool) -> usize {
    if from == to {
        return 1;
    }
    let neighbors = connections.get(from).unwrap();
    let visits = visited.entry(from).or_insert(0);
    *visits += 1;

    neighbors.iter()
             .filter(|neighbor| visiting_allowed(visited, neighbor, allow_twice))
             .map(|neighbor| count_paths(connections, &mut visited.clone(), neighbor, to, allow_twice))
             .sum()
}

fn paths_through_caves<T: AsRef<str>>(input: &[T], allow_twice: bool) -> usize {
    let mut connections = HashMap::new();

    for line in input {
        let (start, end) = line.as_ref().split_once('-').unwrap();

        let conn1 = connections.entry(start).or_insert_with(HashSet::new);
        conn1.insert(end);

        let conn2 = connections.entry(end).or_insert_with(HashSet::new);
        conn2.insert(start);
    }

    let mut visited = HashMap::new();
    count_paths(&connections, &mut visited, "start", "end", allow_twice)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let input = [
            "start-A",
            "start-b",
            "A-c",
            "A-b",
            "b-d",
            "A-end",
            "b-end",
        ];
        assert_eq!(paths_through_caves(&input, false), 10);
        assert_eq!(paths_through_caves(&input, true), 36);
    }

    #[test]
    fn test1() {
        let input = [
            "dc-end",
            "HN-start",
            "start-kj",
            "dc-start",
            "dc-HN",
            "LN-dc",
            "HN-end",
            "kj-sa",
            "kj-HN",
            "kj-dc",
        ];
        assert_eq!(paths_through_caves(&input, false), 19);
    }

    #[test]
    fn test2() {
        let input = [
            "fs-end",
            "he-DX",
            "fs-he",
            "start-DX",
            "pj-DX",
            "end-zg",
            "zg-sl",
            "zg-pj",
            "pj-he",
            "RW-he",
            "fs-DX",
            "pj-RW",
            "zg-RW",
            "start-pj",
            "he-WI",
            "zg-he",
            "pj-fs",
            "start-RW"
        ];
        assert_eq!(paths_through_caves(&input, false), 226);
    }
}
