use std::{cmp::min, ops::Range};

struct Map {
    range: Range<isize>,
    increment: isize,
}

pub fn solve1(input: Vec<String>) {
    let input_string = input.join("\n");
    let input_segments: Vec<&str> = input_string.split("\n\n").collect();

    let seed_segment = input_segments[0].replace("seeds: ", " ");
    let seed_list: Vec<isize> = seed_segment
        .trim()
        .split(' ')
        .filter(|number| !number.is_empty())
        .map(|seed| seed.parse::<isize>().unwrap())
        .collect();

    let mut map_list: Vec<Vec<Map>> = Vec::new();

    for map_index in 1..input_segments.len() {
        let map_split: Vec<&str> = input_segments[map_index].split("\n").collect();

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
