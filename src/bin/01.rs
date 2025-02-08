use basic_trie::DataTrie;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .trim()
        .lines()
        .map(|f| {
            let numeric_chars: Vec<char> = f.chars().filter(|f| f.is_numeric()).collect();
            let parts = vec![
                numeric_chars.first().unwrap().to_string(),
                numeric_chars.last().unwrap().to_string(),
            ];
            let result: String = parts.join("");
            dbg!(&result);
            result.parse::<u32>().unwrap()
        })
        .sum::<u32>();

    Some(result)
}

// three
// seven
// eight
// four
// five
// nine
// one
// two
// six
pub fn part_two(input: &str) -> Option<u32> {
    let replacements = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let _: Vec<()> = input
        .trim()
        .lines()
        .map(|line| {
            dbg!(&line);
            let it = (0..line.len())
                .filter_map(|index| {
                    let reduced_line = &line[index..];
                    let result = replacements.iter().find_map(|(start_str, val)| {
                        if reduced_line.starts_with(start_str) {
                            Some(val.to_string())
                        } else {
                            reduced_line.chars().next().map(|c| c.to_string())
                        }
                    });
                    let result = result.and_then(|s| s.parse::<u32>().ok());
                    dbg!(&result);
                    result
                })
                .collect::<Vec<u32>>();
            // let mut numeric_chars: Vec<String> = vec![];
            // let mut iter = line
            dbg!(it);
        })
        .collect();

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
