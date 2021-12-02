
pub fn read_file(day: u8) -> String {
    let filename = format!("inputs/day{}", day);
    std::fs::read_to_string(filename).unwrap()
}

pub fn read_lines(day: u8) -> Vec<String> {
    read_file(day).split('\n')
                  .filter(|n| !n.is_empty())
                  .map(String::from)
                  .collect()
}

pub fn read_numbers(day: u8) -> Vec<usize> {
    read_lines(day).iter()
                   .map(|n| n.parse::<usize>().unwrap())
                   .collect()
}
