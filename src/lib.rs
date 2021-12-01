
pub fn read_file(day: u8) -> String {
    let filename = format!("inputs/day{}", day);
    std::fs::read_to_string(filename).unwrap()
}

pub fn read_numbers(day: u8) -> Vec<usize> {
    read_file(day).split('\n')
                  .filter(|n| !n.is_empty())
                  .map(|n| n.parse::<usize>().unwrap())
                  .collect()
}
