extern crate helpers;

use std::collections::HashMap;

use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use petgraph::Graph;

use helpers::{Coord, Grid};

fn get_elevation(elevation: char) -> u8 {
    match elevation {
        'S' => 0,
        'E' => 'z' as u8 - 'a' as u8,
        _ => elevation as u8 - 'a' as u8,
    }
}

fn make_graph(input: &Grid<char>) -> (HashMap<Coord, NodeIndex>, Graph<u8, ()>) {
    let mut graph = Graph::new();
    let mut nodes = HashMap::new();

    for (coord, elevation) in input.iter() {
        nodes.insert(coord, graph.add_node(get_elevation(*elevation)));
    }

    for (coord, node_index) in nodes.iter() {
        let elevation = graph[*node_index];

        for (coord, neighbour_elevation) in input.neighbors_iter(*coord, false) {
            if get_elevation(*neighbour_elevation) as isize - elevation as isize <= 1 {
                graph.add_edge(*node_index, *nodes.get(&coord).unwrap(), ());
            }
        }
    }

    (nodes, graph)
}

fn find_lower_steps(graph: &Graph<u8, ()>, start: NodeIndex, dest: NodeIndex) -> Option<usize> {
    astar(graph, start, |_dest| dest == _dest, |_| 1, |_| 0).map(|(steps, _)| steps)
}

pub fn part1(input: &Grid<char>) -> usize {
    let current_position = input
        .iter()
        .find(|(_, &c)| c == 'S')
        .map(|(position, _)| position)
        .unwrap();
    let end_position = input
        .iter()
        .find(|(_, &c)| c == 'E')
        .map(|(position, _)| position)
        .unwrap();

    let (nodes, graph) = make_graph(input);

    let start = *nodes.get(&current_position).unwrap();
    let dest = *nodes.get(&end_position).unwrap();

    find_lower_steps(&graph, start, dest).unwrap()
}

pub fn part2(input: &Grid<char>) -> usize {
    let lowest_positions = input
        .iter()
        .filter(|(_, &c)| c == 'S' || c == 'a')
        .map(|(position, _)| position)
        .collect::<Vec<Coord>>();
    let end_position = input
        .iter()
        .find(|(_, &c)| c == 'E')
        .map(|(position, _)| position)
        .unwrap();

    let (nodes, graph) = make_graph(input);

    lowest_positions
        .iter()
        .filter_map(|position| {
            let start = *nodes.get(&position).unwrap();
            let dest = *nodes.get(&end_position).unwrap();

            find_lower_steps(&graph, start, dest)
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use helpers::input_grid;

    use super::*;

    fn input<'a>() -> Grid<char> {
        let input = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        input_grid(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 31)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 29)
    }
}
