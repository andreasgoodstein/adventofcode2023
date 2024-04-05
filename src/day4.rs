use std::collections::{HashMap, HashSet};

struct Card {
    id: usize,
    own_numbers: HashSet<usize>,
    winning_numbers: HashSet<usize>,
}

pub fn solve1(input: Vec<String>) {
    let mut total = 0;

    for line in input {
        let card = parse_card(line);

        total += calculate_card_score(&card);
    }

    println!("{total}")
}

fn parse_card(input: String) -> Card {
    let split: Vec<&str> = input.split(':').collect();

    let id_split: Vec<&str> = split[0].split(' ').collect();
    let numbers_split: Vec<&str> = split[1].split('|').collect();

    let id = id_split.last().unwrap().trim().parse::<usize>().unwrap();

    let own_numbers = parse_string_list_to_number_list(numbers_split[1]);
    let winning_numbers = parse_string_list_to_number_list(numbers_split[0]);

    Card {
        id: id,
        own_numbers: HashSet::from_iter(own_numbers),
        winning_numbers: HashSet::from_iter(winning_numbers),
    }
}

fn parse_string_list_to_number_list(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(' ')
        .filter(|number| !number.is_empty())
        .map(|number| number.parse::<usize>().unwrap())
        .collect()
}

fn calculate_card_score(card: &Card) -> usize {
    let overlap: u32 =
        u32::try_from(card.winning_numbers.intersection(&card.own_numbers).count()).unwrap();

    match overlap {
        0 => 0,
        _ => usize::pow(2, overlap - 1),
    }
}

pub fn solve2(input: Vec<String>) {
    let max_card_id = input.len();
    let mut card_map: HashMap<usize, usize> = HashMap::new();

    for line in input {
        let card = parse_card(line);

        let new_card_count = card_map.get(&card.id).unwrap_or(&0) + 1;

        card_map.insert(card.id, new_card_count);

        let winning_numbers = calculate_card_winning_numbers(&card);

        if winning_numbers == 0 {
            continue;
        }

        for increment in 1..winning_numbers + 1 {
            let card_copy_id = card.id + increment;
            if card_copy_id > max_card_id {
                break;
            }

            let new_card_copy_count = card_map.get(&card_copy_id).unwrap_or(&0) + new_card_count;

            card_map.insert(card_copy_id, new_card_copy_count);
        }
    }

    println!("{}", card_map.into_values().sum::<usize>());
}

fn calculate_card_winning_numbers(card: &Card) -> usize {
    card.winning_numbers.intersection(&card.own_numbers).count()
}
