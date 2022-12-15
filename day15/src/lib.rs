use std::ops::RangeInclusive;

use regex::Regex;

struct Coord(isize, isize);

impl Coord {
    fn distance(&self, other: &Coord) -> isize {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

struct Sensor {
    coord: Coord,
    beacon: Coord,
}

fn parse_sensors(input: &Vec<&str>) -> Vec<Sensor> {
    let regex = Regex::new(r"Sensor at x=(?P<x1>-?\d+), y=(?P<y1>-?\d+): closest beacon is at x=(?P<x2>-?\d+), y=(?P<y2>-?\d+)").unwrap();

    input
        .iter()
        .map(|line| {
            let captures = regex.captures(line).unwrap();

            Sensor {
                coord: Coord(
                    captures["x1"].parse().unwrap(),
                    captures["y1"].parse().unwrap(),
                ),
                beacon: Coord(
                    captures["x2"].parse().unwrap(),
                    captures["y2"].parse().unwrap(),
                ),
            }
        })
        .collect()
}

fn calc_sensors_ranges_for_row(sensors: &Vec<Sensor>, row: isize) -> Vec<RangeInclusive<isize>> {
    let mut ranges = vec![];

    for sensor in sensors {
        let distance = sensor.coord.distance(&sensor.beacon);
        let distance_to_row = sensor.coord.distance(&Coord(sensor.coord.0, row));

        if distance_to_row > distance {
            continue;
        }

        let diff = distance - distance_to_row;

        ranges.push((sensor.coord.0 - diff)..=(sensor.coord.0 + diff));
    }

    ranges.sort_by(|a, b| a.start().cmp(&b.start()));

    ranges
}

pub fn part1(input: &Vec<&str>, row: isize) -> isize {
    let sensors = parse_sensors(input);
    let ranges = calc_sensors_ranges_for_row(&sensors, row);
    let mut max = ranges[0].end();
    let mut count = ranges[0].end() - ranges[0].start();

    for range in ranges.iter().skip(1) {
        if range.start() - max > 1 {
            count += range.end() - range.start();
        } else if range.end() > max {
            count += range.end() - max;
        }

        max = max.max(range.end());
    }

    count
}

pub fn part2(input: &Vec<&str>, min: isize, max: isize) -> isize {
    let sensors = parse_sensors(input);

    for y in min..=max {
        let ranges = calc_sensors_ranges_for_row(&sensors, y);
        let mut max = ranges[0].end();

        for range in ranges.iter().skip(1) {
            if range.start() - max > 1 {
                let x = range.start() - 1;
                return x * 4000000 + y;
            }

            max = max.max(range.end());
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input(), 10), 26)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input(), 0, 20), 56000011)
    }
}
