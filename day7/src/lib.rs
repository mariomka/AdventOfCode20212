use std::collections::HashMap;

use itertools::Itertools;

fn join_path(path: String, join: &str) -> String {
    if path == "/" {
        path + join
    } else {
        path + "/" + join
    }
}

fn calc_directory_sizes(input: &Vec<&str>) -> HashMap<String, usize> {
    let mut directory_sizes: HashMap<String, usize> = HashMap::new();
    let mut current_directory_path = String::new();

    for line in input {
        // command
        let split = line.rsplit_once(' ').unwrap();

        match split.0 {
            "$ cd" => {
                // change directory
                current_directory_path = match split.1 {
                    "/" => "/".to_string(),
                    ".." => {
                        let path = current_directory_path.rsplit_once('/').unwrap().0;

                        if path.is_empty() {
                            "/".to_string()
                        } else {
                            path.to_string()
                        }
                    }
                    _ => join_path(current_directory_path, split.1),
                };
            }
            "$" => {
                // list directory - do nothing
            }
            "dir" => {
                // directory - do nothing
            }
            _ => {
                // file
                let file_size = split.0.parse::<usize>().unwrap();
                let mut current = "/".to_string();

                for part in current_directory_path.split('/').dedup() {
                    current = join_path(current, part);
                    directory_sizes
                        .entry(current.clone())
                        .and_modify(|directory_size| *directory_size += file_size)
                        .or_insert(file_size);
                }
            }
        }
    }

    directory_sizes
}

pub fn part1(input: &Vec<&str>) -> usize {
    calc_directory_sizes(input)
        .into_values()
        .filter(|size| *size <= 100000)
        .sum()
}

pub fn part2(input: &Vec<&str>) -> usize {
    let directory_sizes = calc_directory_sizes(input);
    let root_size = *directory_sizes.get("/").unwrap() as isize;
    let size_to_free = 30000000 - (70000000 - root_size) as usize;

    directory_sizes
        .into_values()
        .filter(|size| *size >= size_to_free)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 95437)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 24933642)
    }
}
