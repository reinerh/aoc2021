use std::collections::VecDeque;
use std::collections::HashMap;

fn main() {
    let input = advent::read_lines(10);
    println!("10a: {}", syntax_error_score(&input));
    println!("10b: {}", completion_error_score(&input));
}

enum ParsingError {
    Corrupt { illegal: char },
    Incomplete { missing: VecDeque<char> },
}

fn parse_line(line: &str) -> Result<(), ParsingError> {
    let mut openings = VecDeque::new();
    let chunks = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]);

    for c in line.chars() {
        if chunks.contains_key(&c) {
            openings.push_front(c);
        } else {
            if openings.is_empty() {
                return Err(ParsingError::Corrupt { illegal: c });
            }
            let expected = *chunks.get(openings.front().unwrap()).unwrap();
            if expected != c {
                return Err(ParsingError::Corrupt { illegal: c });
            } else {
                openings.pop_front();
            }
        }
    }

    if !openings.is_empty() {
        Err(ParsingError::Incomplete { missing: openings })
    } else {
        Ok(())
    }
}

fn syntax_error_score<T: AsRef<str>>(input: &[T]) -> usize {
    let scores_corrupt = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);

    let mut score = 0;
    for line in input {
        if let Err(ParsingError::Corrupt { illegal }) = parse_line(line.as_ref()) {
            score += scores_corrupt.get(&illegal).unwrap();
        }
    }
    score
}

fn completion_error_score<T: AsRef<str>>(input: &[T]) -> usize {
    let scores_incomplete = HashMap::from([
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4),
    ]);

    let mut scores = Vec::new();
    for line in input {
        if let Err(ParsingError::Incomplete { missing }) = parse_line(line.as_ref()) {
            let mut score = 0;
            for c in missing {
                score *= 5;
                score += scores_incomplete.get(&c).unwrap();
            }
            scores.push(score);
        }
    }
    scores.sort_unstable();
    scores[scores.len()/2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ];
        assert_eq!(syntax_error_score(&input), 26397);
        assert_eq!(completion_error_score(&input), 288957);
    }
}
