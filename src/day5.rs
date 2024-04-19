use std::{cmp::max, cmp::min, ops::Range};

#[derive(Clone)]
struct Map {
    start: isize,
    end: isize,
    increment: isize,
}

pub fn solve1(input: Vec<String>) {
    let input_string = input.join("\n");
    let input_segments: Vec<&str> = input_string.split("\n\n").collect();

    let seed_list = parse_seed_list(input_segments[0]);

    let map_list = parse_map_list(input_segments);

    let min_location = find_min_location(seed_list, map_list);

    println!("{min_location}")
}

pub fn solve2(input: Vec<String>) {
    let input_string = input.join("\n");
    let input_segments: Vec<&str> = input_string.split("\n\n").collect();

    let mut seed_range_list = parse_seed_range_list(input_segments[0]);

    let map_list_list = parse_map_list(input_segments);

    for map_list in map_list_list {
        seed_range_list = seed_range_list
            .into_iter()
            .flat_map(|seed_range| split_range_with_map(&seed_range, &map_list))
            .collect();
    }

    let mut min_range = Range {
        start: isize::MAX,
        end: 0,
    };

    for range in seed_range_list {
        if range.start < min_range.start {
            min_range = range;
        }
    }

    println!("{}", min_range.start);
}

fn split_range_with_map(range: &Range<isize>, map_list: &Vec<Map>) -> Vec<Range<isize>> {
    if map_list.len() < 1 {
        return Vec::from([Range {
            start: range.start,
            end: range.end,
        }]);
    }

    let map = map_list.first().unwrap();
    let map_list_rest: &Vec<Map> = &map_list.to_vec().into_iter().skip(1).collect();

    if range.end < map.start || map.end < range.start {
        return split_range_with_map(range, map_list_rest);
    }

    if map.start <= range.start && range.end <= map.end {
        return Vec::from([Range {
            start: range.start + map.increment,
            end: range.end + map.increment,
        }]);
    }

    let mut new_range_list: Vec<Range<isize>> = Vec::new();

    if range.start < map.start {
        let mut recursive_list = split_range_with_map(
            &Range {
                start: range.start,
                end: map.start,
            },
            map_list_rest,
        );
        new_range_list.append(&mut recursive_list)
    }

    if range.end > map.end {
        let mut recursive_list = split_range_with_map(
            &Range {
                start: map.end,
                end: range.end,
            },
            map_list_rest,
        );
        new_range_list.append(&mut recursive_list)
    }

    new_range_list.push(Range {
        start: max(range.start, map.start) + map.increment,
        end: min(range.end, map.end) + map.increment,
    });

    new_range_list
}

fn find_min_location(seed_list: Vec<isize>, map_list: Vec<Vec<Map>>) -> isize {
    let mut min_location = isize::MAX;

    for seed in seed_list {
        let mut location = seed;
        for outer_map in &map_list {
            for inner_map in outer_map {
                if inner_map.start < location && inner_map.end > location {
                    location = location + inner_map.increment;
                    break;
                }
            }
        }

        min_location = min(location, min_location);
    }

    min_location
}

fn find_min_range() {}

fn parse_seed_range_list(input: &str) -> Vec<Range<isize>> {
    let seed_segment = input.replace("seeds: ", "");

    let range_split: Vec<&str> = seed_segment.split(' ').collect();

    let mut range_list: Vec<Range<isize>> = Vec::new();

    for i in (0..range_split.len()).step_by(2) {
        let start = range_split[i].parse::<isize>().unwrap();
        let end = range_split[i + 1].parse::<isize>().unwrap() + start;

        range_list.push(Range { start, end });
    }

    range_list
}

fn parse_seed_list(input: &str) -> Vec<isize> {
    let seed_segment = input.replace("seeds: ", "");

    seed_segment
        .trim()
        .split(' ')
        .filter(|number| !number.is_empty())
        .map(|seed| seed.parse::<isize>().unwrap())
        .collect()
}

fn parse_map_list(segments: Vec<&str>) -> Vec<Vec<Map>> {
    let mut map_list: Vec<Vec<Map>> = Vec::new();

    for map_index in 1..segments.len() {
        let map_split: Vec<&str> = segments[map_index].split("\n").collect();

        let mut new_map_list: Vec<Map> = Vec::new();
        for map_string in map_split {
            if map_string.contains("map:") {
                continue;
            }

            let map_number_list: Vec<isize> = map_string
                .trim()
                .split(' ')
                .filter(|number| !number.is_empty())
                .map(|number| number.parse::<isize>().unwrap())
                .collect();

            let destination_index = map_number_list[0];
            let source_index = map_number_list[1];
            let map_count = map_number_list[2];

            new_map_list.push(Map {
                end: source_index + map_count,
                start: source_index,
                increment: destination_index - source_index,
            });
        }

        map_list.push(new_map_list);
    }

    map_list
}
