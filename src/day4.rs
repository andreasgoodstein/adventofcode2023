use std::collections::HashSet;

struct Card {
    // id: usize,
    own_numbers: HashSet<usize>,
    winning_numbers: HashSet<usize>,
}

pub fn solve1(input: Vec<String>) {
    let mut total = 0;

    for line in input {
        let card = parse_card(line);

        total += calculate_card_score(card);
    }

    println!("{total}")
}

fn parse_card(input: String) -> Card {
    let id_split: Vec<&str> = input.split(':').collect();

    // let id_split: Vec<&str> = id_split[0].split(' ').collect();
    let numbers_split: Vec<&str> = id_split[1].split('|').collect();

    // let id = id_split[1].trim().parse::<usize>().unwrap();

    let own_numbers = parse_string_list_to_number_list(numbers_split[1]);
    let winning_numbers = parse_string_list_to_number_list(numbers_split[0]);

    Card {
        // id: id,
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

fn calculate_card_score(card: Card) -> usize {
    let overlap: u32 =
        u32::try_from(card.winning_numbers.intersection(&card.own_numbers).count()).unwrap();

    match overlap {
        0 => 0,
        _ => usize::pow(2, overlap - 1),
    }
}
