fn main() {
    let input = advent::read_lines(3);
    println!("3a: {}", power_consumption(&input));
    println!("3b: {}", life_support_rating(&input));
}


fn power_consumption<T: AsRef<str>>(input: &[T]) -> u32 {
    let bits = count_ones(input);
    let gamma = rate(&bits, input.len(), true);
    let epsilon = rate(&bits, input.len(), false);

    gamma * epsilon
}

fn count_ones_at_pos<T: AsRef<str>>(input: &[T], pos: usize) -> usize {
    input.iter()
         .filter(|&x| x.as_ref().chars().nth(pos).unwrap() == '1')
         .count()
}

fn count_ones<T: AsRef<str>>(input: &[T]) -> Vec<usize> {
    input[0].as_ref()
            .chars()
            .enumerate()
            .map(|(i, _)| count_ones_at_pos(input, i))
            .collect()
}

fn rate(bits: &[usize], max_count: usize, gamma: bool) -> u32 {
    let mut result = 0;
    let max_count = max_count;

    for count in bits.iter() {
        result <<= 1;
        if gamma && 2 * count > max_count {
            result += 1;
        }
        if !gamma && 2 * count < max_count {
            result += 1;
        }
    }
    result
}

fn life_support_rating<T: AsRef<str>>(input: &[T]) -> usize {
    let oxygen = oxygen_rating(input);
    let co2 = co2_rating(input);

    oxygen * co2
}

fn co2_rating<T: AsRef<str>>(input: &[T]) -> usize {
    rating(input, false)
}

fn oxygen_rating<T: AsRef<str>>(input: &[T]) -> usize {
    rating(input, true)
}

fn rating<T: AsRef<str>>(input: &[T], oxygen: bool) -> usize {
    let mut numbers = vec!["0"; input.len()];
    for (i, number) in input.iter().enumerate() {
        numbers[i] = number.as_ref();
    }

    let len = numbers[0].len();
    for i in 0 .. len {
        let ones = count_ones_at_pos(&numbers, i);
        let keep = if (oxygen && 2 * ones >= numbers.len()) || (!oxygen && 2 * ones < numbers.len()) {
            '1'
        } else {
            '0'
        };
        numbers = numbers.iter()
                         .filter(|&x| x.chars().nth(i).unwrap() == keep)
                         .copied()
                         .collect();
        if numbers.len() == 1 {
            break;
        }
    }
    usize::from_str_radix(numbers[0], 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010",
        ];
        assert_eq!(power_consumption(&input), 198);
        assert_eq!(life_support_rating(&input), 230);
    }
}
