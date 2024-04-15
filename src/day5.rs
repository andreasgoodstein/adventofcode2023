use std::{cmp::min, ops::Range};

struct Map {
    range: Range<isize>,
    increment: isize,
}

pub fn solve1(input: Vec<String>) {
    let input_string = input.join("\n");
    let input_segments: Vec<&str> = input_string.split("\n\n").collect();

    let seed_list = parse_seed_list(input_segments[0]);

    let map_list = parse_map_list(input_segments);

    let mut min_location = isize::MAX;

    for seed in seed_list {
        let mut distance = seed;
        for outer_map in &map_list {
            for inner_map in outer_map {
                if inner_map.range.contains(&distance) {
                    distance = distance + inner_map.increment;
                    break;
                }
            }
        }
        min_location = min(distance, min_location);
    }

    println!("{min_location}")
}

fn parse_seed_list(input: &str) -> Vec<isize> {
    let seed_segment = input.replace("seeds: ", " ");

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
                range: Range {
                    end: source_index + map_count,
                    start: source_index,
                },
                increment: destination_index - source_index,
            });
        }

        map_list.push(new_map_list);
    }

    map_list
}