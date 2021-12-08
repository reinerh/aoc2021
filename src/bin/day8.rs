use std::collections::HashSet;

fn main() {
    let input = advent::read_lines(8);
    println!("8a: {}", count_unique_digits(&input));
    println!("8b: {}", decode_digits(&input));
}

fn count_unique_digits<T: AsRef<str>>(input: &[T]) -> usize {
    let mut sum = 0;
    for line in input {
        let segments = line.as_ref()
                           .split_once(" | ")
                           .map(|(_, s)| s)
                           .unwrap()
                           .split_terminator(' ')
                           .filter(|segment| [2,3,4,7].contains(&segment.len()))
                           .count();
        sum += segments;
    }
    sum
}

#[derive(Default)]
struct SevenSegment {
    top: Option<char>,
    middle: Option<char>,
    bottom: Option<char>,
    topleft: Option<char>,
    topright: Option<char>,
    bottomleft: Option<char>,
    bottomright: Option<char>,
}

fn deduce_decoder(signals: &[&str]) -> SevenSegment {
    let mut decoder = SevenSegment::default();

    let mut segment_counts : Vec<Vec<HashSet<char>>> = Vec::new();
    for i in 0..10 {
        let vec = signals.iter()
                         .filter(|s| s.len() == i)
                         .map(|&s| s.chars().collect())
                         .collect();
        segment_counts.push(vec);
    }

    /* top element is difference between 7 and 1 */
    decoder.top = Some(*segment_counts[3][0].difference(&segment_counts[2][0]).next().unwrap());

    /* top right element is difference between 8 and 6 */
    for s in &segment_counts[6] {
        /* possible differences: 0, 6, 9 */
        let diff = *segment_counts[7][0].difference(s).next().unwrap();
        /* if the single different segment is also in 1, the checked segment is 6 */
        if segment_counts[2][0].contains(&diff) {
            decoder.topright = Some(diff);
        }
    }

    /* with top and topright segments we can find bottomright segment via 7 */
    let mut tmp = HashSet::new();
    tmp.insert(decoder.top.unwrap());
    tmp.insert(decoder.topright.unwrap());
    decoder.bottomright = Some(*segment_counts[3][0].difference(&tmp).next().unwrap());

    /* segments of 4 and 7 are contained in 9 */
    for s in &segment_counts[6] {
        if segment_counts[4][0].is_subset(s) {
            let diff94 : HashSet<char> = s.difference(&segment_counts[4][0]).cloned().collect();
            let diff947 = *diff94.difference(&segment_counts[3][0]).next().unwrap();
            decoder.bottom = Some(diff947);

            /* bottomleft is difference between 8 and 9 */
            decoder.bottomleft = Some(*segment_counts[7][0].difference(s).next().unwrap());
        }
    }

    /* 3 is the only 5-segment digit that fully contains 7 */
    for s in &segment_counts[5] {
        if segment_counts[3][0].is_subset(s) {
            let mut tmp = segment_counts[3][0].clone();
            tmp.insert(decoder.bottom.unwrap());

            decoder.middle = Some(*s.difference(&tmp).next().unwrap());
        }
    }

    /* topleft can be found with 4 and known decodings */
    let mut tmp = HashSet::new();
    tmp.insert(decoder.topright.unwrap());
    tmp.insert(decoder.bottomright.unwrap());
    tmp.insert(decoder.middle.unwrap());
    decoder.topleft = Some(*segment_counts[4][0].difference(&tmp).next().unwrap());

    decoder
}

fn possible_digits(decoder: &SevenSegment) -> Vec<HashSet<char>> {
    let set0 = HashSet::from([
        decoder.top.unwrap(), decoder.topright.unwrap(), decoder.bottomright.unwrap(),
        decoder.bottom.unwrap(), decoder.bottomleft.unwrap(), decoder.topleft.unwrap(),
    ]);
    let set1 = HashSet::from([
        decoder.topright.unwrap(), decoder.bottomright.unwrap(),
    ]);
    let set2 = HashSet::from([
        decoder.top.unwrap(), decoder.topright.unwrap(), decoder.middle.unwrap(),
        decoder.bottomleft.unwrap(), decoder.bottom.unwrap(),
    ]);
    let set3 = HashSet::from([
        decoder.top.unwrap(), decoder.topright.unwrap(), decoder.bottomright.unwrap(),
        decoder.middle.unwrap(), decoder.bottom.unwrap(),
    ]);
    let set4 = HashSet::from([
        decoder.topleft.unwrap(), decoder.topright.unwrap(),
        decoder.middle.unwrap(), decoder.bottomright.unwrap(),
    ]);
    let set5 = HashSet::from([
        decoder.top.unwrap(), decoder.topleft.unwrap(), decoder.middle.unwrap(),
        decoder.bottomright.unwrap(), decoder.bottom.unwrap(),
    ]);
    let set6 = HashSet::from([
        decoder.top.unwrap(), decoder.topleft.unwrap(), decoder.middle.unwrap(),
        decoder.bottomright.unwrap(), decoder.bottom.unwrap(), decoder.bottomleft.unwrap(),
    ]);
    let set7 = HashSet::from([
        decoder.top.unwrap(), decoder.topright.unwrap(), decoder.bottomright.unwrap(),
    ]);
    let set8 = HashSet::from([
        decoder.top.unwrap(), decoder.topleft.unwrap(), decoder.topright.unwrap(), decoder.middle.unwrap(),
        decoder.bottomright.unwrap(), decoder.bottom.unwrap(), decoder.bottomleft.unwrap(),
    ]);
    let set9 = HashSet::from([
        decoder.top.unwrap(), decoder.topleft.unwrap(), decoder.topright.unwrap(), decoder.middle.unwrap(),
        decoder.bottom.unwrap(), decoder.bottomright.unwrap(),
    ]);

    Vec::from([set0, set1, set2, set3, set4, set5, set6, set7, set8, set9])
}

fn decode_digit(decoder: &SevenSegment, input: &str) -> usize {
    let possible = possible_digits(decoder);
    let segments : HashSet<char> = input.chars().collect();

    possible.iter()
            .enumerate()
            .filter(|&(_, p)| segments == *p)
            .map(|(i, _)| i)
            .next()
            .unwrap()
}

fn decode_digits<T: AsRef<str>>(input: &[T]) -> usize {
    let mut number_sum = 0;
    for line in input {
        let (signals, segments) = line.as_ref()
                                      .split_once(" | ")
                                      .unwrap();
        let signals : Vec<&str> = signals.split_terminator(' ').collect();
        let segments : Vec<&str> = segments.split_terminator(' ').collect();

        let decoder = deduce_decoder(&signals);

        let mut number = 0;
        for s in segments {
            number *= 10;
            number += decode_digit(&decoder, s);
        }
        number_sum += number;
    }
    number_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
        ];
        assert_eq!(count_unique_digits(&input), 26);
        assert_eq!(decode_digits(&input), 61229);
    }
}
