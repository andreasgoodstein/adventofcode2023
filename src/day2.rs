use std::cmp;
use std::fmt;

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

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}|{}|{}", self.blue, self.green, self.red)
    }
}

pub fn solve1(input: Vec<String>) {
    let mut sum: usize = 0;

    for line in input {
        let game: Game = parse_game(line);

        let mut discontinue = false;

        for hand in game.hands {
            if hand.blue > MAX_BLUE {
                discontinue = true;
                break;
            }
            if hand.green > MAX_GREEN {
                discontinue = true;
                break;
            }
            if hand.red > MAX_RED {
                discontinue = true;
                break;
            }
        }

        if discontinue {
            continue;
        }

        sum += game.id;
    }

    println!("{sum}")
}

pub fn solve2(input: Vec<String>) {
    let mut sum: usize = 0;

    let game_list: Vec<Game> = input.into_iter().map(|line| parse_game(line)).collect();

    for game in game_list {
        let mut min_blue: usize = 0;
        let mut min_green: usize = 0;
        let mut min_red: usize = 0;

        for hand in game.hands {
            min_blue = cmp::max(min_blue, hand.blue);
            min_green = cmp::max(min_green, hand.green);
            min_red = cmp::max(min_red, hand.red);
        }

        let hand_power = min_blue * min_green * min_red;
        sum += hand_power;
    }

    println!("{sum}")
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
            let cube_fragments: Vec<&str> = cube.trim().split(' ').collect();
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
