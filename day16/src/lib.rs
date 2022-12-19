use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use regex::Regex;

type Label = String;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Valve {
    flow_rate: usize,
    tunnels: Vec<Label>,
}

fn parse_valves(input: &Vec<&str>) -> HashMap<Label, Valve> {
    let regex = Regex::new(r"Valve (?P<label>[A-Z]{2}) has flow rate=(?P<flow_rate>\d+); tunnels? leads? to valves? (?P<tunnels>.+)").unwrap();

    let mut valves: HashMap<Label, Valve> = HashMap::new();

    for line in input {
        let captures = regex.captures(line).unwrap();

        valves.insert(
            captures["label"].to_string(),
            Valve {
                flow_rate: captures["flow_rate"].parse().unwrap(),
                tunnels: captures["tunnels"]
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect(),
            },
        );
    }
    return valves;
}

struct Item {
    label: Label,
    opened: HashSet<Label>,
    minutes: usize,
    flow: usize,
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct Vertex {
    label: Label,
    dist: usize,
}

impl Vertex {
    fn new(label: Label, dist: usize) -> Vertex {
        Vertex { label, dist }
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.dist.cmp(&self.dist))
    }
}

fn find_shortest_paths(start: Label, valves: &HashMap<Label, Valve>) -> HashMap<Label, Vec<Label>> {
    let mut dist = HashMap::new();

    let mut prev = HashMap::new();

    let mut queue: BinaryHeap<Vertex> = BinaryHeap::new();

    dist.insert(start.clone(), 0);
    queue.push(Vertex::new(start.clone(), 0));

    for (label, _) in valves.iter() {
        if *label == start {
            continue;
        }
        dist.insert(label.to_string(), usize::MAX);
        // prev[v] ← UNDEFINED
        queue.push(Vertex::new(label.clone(), usize::MAX));
    }

    while let Some(vertex) = queue.pop() {
        let valve = valves.get(&vertex.label).unwrap();

        for tunnel in valve.tunnels.iter() {
            let alt = dist[&vertex.label] + 1;

            if alt < dist[tunnel] {
                dist.insert(tunnel.to_string(), alt);
                prev.insert(tunnel.to_string(), vertex.label.clone());
            }
        }

        let mut new_queue = BinaryHeap::new();
        queue.iter().for_each(|vertex| {
            new_queue.push(Vertex::new(vertex.label.clone(), dist[&vertex.label]))
        });
        queue = new_queue;
    }

    let mut paths = HashMap::new();

    for (label, _) in valves.iter() {
        if *label == start {
            continue;
        }

        let mut path = vec![label.clone()];
        let mut current = label.clone();

        loop {
            let new_current = prev.get(&current).unwrap().clone();

            if new_current == start {
                break;
            }

            path.push(new_current.clone());
            current = new_current;
        }
        path.reverse();
        paths.insert(label.clone(), path);
    }

    paths
}

pub fn part1(input: &Vec<&str>) -> usize {
    let valves = parse_valves(input);
    let mut shortest_paths = HashMap::new();

    for (label, _) in valves.iter() {
        shortest_paths.insert(label.clone(), find_shortest_paths(label.clone(), &valves));
    }

    let mut queue: VecDeque<Item> = VecDeque::new();
    queue.push_back(Item {
        label: "AA".to_string(),
        opened: HashSet::new(),
        minutes: 0,
        flow: 0,
    });

    let mut best_flow = 0;

    while let Some(item) = queue.pop_front() {
        if item.minutes >= 30 {
            if item.flow > best_flow {
                best_flow = item.flow;
            }
            continue;
        }

        let paths = shortest_paths
            .get(&item.label)
            .unwrap()
            .iter()
            .filter(|(to_label, _)| {
                let valve = valves.get(*to_label).unwrap();

                valve.flow_rate > 0 && !item.opened.contains(*to_label)
            })
            .collect::<Vec<_>>();

        if paths.is_empty() {
            queue.push_back(Item {
                label: item.label.clone(),
                opened: item.opened.clone(),
                minutes: 30,
                flow: item.flow,
            });
            continue;
        }

        for (to_label, path) in paths {
            let valve = valves.get(to_label).unwrap();

            if item.minutes + path.len() + 1 <= 30 {
                let mut opened = item.opened.clone();
                opened.insert(to_label.clone());
                let minutes = item.minutes + path.len() + 1;
                let plus_flow = valve.flow_rate * (30 - minutes);
                queue.push_back(Item {
                    label: to_label.clone(),
                    opened,
                    minutes,
                    flow: item.flow + plus_flow,
                });
            } else {
                queue.push_back(Item {
                    label: to_label.clone(),
                    opened: item.opened.clone(),
                    minutes: 30,
                    flow: item.flow,
                });
            }
        }
    }

    best_flow
}

// pub fn part2(input: &Vec<&str>) -> usize {
//     0
// }

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 1651)
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&input()), 1707)
    // }
}
