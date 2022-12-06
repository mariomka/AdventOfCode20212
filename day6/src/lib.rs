fn find_message_marker(input: &str, marker_length: usize) -> usize {
    let chars: Vec<char> = input.chars().collect();

    for i in marker_length - 1..chars.len() {
        let mut packet = chars[i - (marker_length - 1)..=i].to_vec();
        packet.sort_unstable();
        packet.dedup();

        if packet.len() == marker_length {
            return i + 1;
        }
    }

    0
}

pub fn part1(input: &str) -> usize {
    find_message_marker(input, 4)
}

pub fn part2(input: &str) -> usize {
    find_message_marker(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
