struct Hand {
    blue: usize,
    green: usize,
    red: usize,
}

struct Game {
    hands: Vec<Hand>,
    id: usize,
}

const MAX_BLUE: usize = 14;
const MAX_GREEN: usize = 13;
const MAX_RED: usize = 12;

pub fn solve1(input: Vec<String>) {
    for line in input {
        let game = parse_game(line);
        for hand in game.hands {
            if hand.blue > MAX_BLUE {
                println!("Game invalid {} blue {}", game.id, hand.blue);
                break;
            }
            if hand.green > MAX_GREEN {
                println!("Game invalid {} blue {}", game.id, hand.green);
                break;
            }
            if hand.red > MAX_RED {
                println!("Game invalid {} blue {}", game.id, hand.red);
                break;
            }
        }
    }
}

fn parse_game(line: String) -> Game {
    let line_fragments: Vec<&str> = line.split(':').collect();

    let game_id = line_fragments[0]
        .replace("Game ", "")
        .parse::<usize>()
        .unwrap();

    let hands_list: Vec<&str> = line_fragments[1].split(';').collect();

    let mut game = Game {
        hands: Vec::new(),
        id: game_id,
    };

    for hand_string in hands_list {
        let mut hand = Hand {
            blue: 0,
            green: 0,
            red: 0,
        };

        let cube_list: Vec<&str> = hand_string.split(',').collect();
        for cube in cube_list {
            let cube_fragments: Vec<&str> = cube.split(' ').collect();
            let cube_count = cube_fragments[0].parse::<usize>().unwrap();
            let cube_color = cube_fragments[1];

            match cube_color {
                "blue" => {
                    hand.blue = cube_count;
                }
                "green" => {
                    hand.green = cube_count;
                }
                "red" => hand.red = cube_count,
                _ => {}
            }
        }

        game.hands.push(hand)
    }

    game
}
