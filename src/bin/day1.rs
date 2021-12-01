fn main() {
    let input = advent::read_numbers(1);
    println!("1a: {}", count_increasing(&input));
    println!("1b: {}", count_increasing_window(&input, 3));
}

fn count_increasing(numbers: &[usize]) -> usize {
    count_increasing_window(numbers, 1)
}

fn count_increasing_window(numbers: &[usize], window_size: usize) -> usize {
    numbers[window_size..].iter()
                .zip(numbers.iter())
                .map(|(&x,&y)| (x as isize) - (y as isize))
                .filter(|&x| x.is_positive())
                .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let numbers = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_increasing(&numbers), 7);
        assert_eq!(count_increasing_window(&numbers, 3), 5);
    }
}
