mod game;
use game::GameMap::GameMap;
use game::Player::Player;
use std::io;
use std::process::exit;

pub fn player_check(player: &mut Player, game_map: &mut GameMap) {
    println!("| Inspection on your own nation? | y = yes | n = no |");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice = choice.trim();

    if choice == "y" {
        player.inspect();
    } else {
        println!("The leader is confident. No inspection needed.");
    }

    let mut spy_choice = String::new();
    println!("| 1) Spy on a country | 2) Invade a country | 3) Expand military |");
    io::stdin()
        .read_line(&mut spy_choice)
        .expect("Failed to read line");
    let spy_choice = spy_choice.trim();

    let user_choice = match spy_choice.parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Invalid option. Try again.");
            return;
        }
    };
    match user_choice {
        1 => {
            player.spy(game_map);
        }
        2 => {
            game_map.list_countries();

            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");
            let choice = choice.trim();

            let user_choice_conq: usize = match choice.parse() {
                Ok(number) => number,
                Err(_) => {
                    println!("Invalid game option. Try again.");
                    return;
                }
            };

            if user_choice_conq < 1 || user_choice_conq > 4 {
                println!("Invalid game input. Try again");
                return;
            }
            let enemy = game_map.get_country_by_index(user_choice_conq - 1);

            player.conquer_nation(enemy, enemy.get_name().clone());
        }
        3 => {
            player.get_country().add_personel();
        }
        _ => {
            println!("Invalid input. Try again.");
            return;
        }
    }
}

fn main() {
    let mut game_map = GameMap::new();
    let mut player: Player;
    loop {
        println!("| 1) Finland | 2) Sweden | 3) Norway | 4) Denmark |");
        println!("Choose your country: ");

        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        let input = input.trim();

        let user_option: usize = match input.parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Invalid input. Try again");
                continue;
            }
        };

        let index_conversion = match user_option {
            1 => 1,
            2 => 3,
            3 => 2,
            4 => 0,
            _ => {
                println!("Invalid choice. Try again.");
                continue;
            }
        };
        player = Player::new(game_map.get_country_by_index(index_conversion).clone());
        break;
    }

    loop {
        player_check(&mut player, &mut game_map);
        let player_name = player.get_country().get_name().clone();
        game_map.other_countries_turn(player_name);

        let win_condition = game_map.get_countries().iter()
            .filter(|c| c.get_name() != &player_name)
            .all(|c| *c.get_is_conquered());

        if win_condition {
            println!("You have conquered all your targets. Good work, comrade!");
            break;
        }
    }
}
