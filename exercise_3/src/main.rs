mod game;
use game::Player::Player;
use game::GameMap::GameMap;
use std::io;
use std::process::exit;

pub fn player_check(player: &Player, game_map: &GameMap) {
    println!("| Inspection on your own nation? | y = yes | n = no |");
    
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice = choice.trim();

    if choice == "y" {
        player.inspect();
    }
    else {
        println!("The leader is confident. No inspection needed.");
    }

    println!("| 1) Spy on a country | 0) Exit program |");
    io::stdin()
        .read_line(&mut String::from(choice))
        .expect("Failed to read line");
    let choice = choice.trim();

    let mut user_choice = match choice.parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Invalid option. Try again.");
            return;
        },
    };
    match user_choice {
        1 => {
            player.spy(game_map);
        },
        0 => { exit(0); },
        _ => {
            println!("Invalid input. Try again.");
            continue;
        }
    }

}

fn main() {
    let mut game_map = GameMap::new();
    loop {
        let player: Player;
        println!("Choose your country: ");
        println!("| 1) Finland | 2) Sweden | 3) Norway | 4) Denmark |");

        let mut input:String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        let input = input.trim();

        let user_option:i16 = match input.parse() {
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
            },
        };

        match index_conversion {
            1 => {
                player = Player::new(game_map.get_country_by_index(user_option).clone());
                break;
            },
            2 => {
                player = Player::new(game_map.get_country_by_index(user_option).clone());
                break;  
            },
            3 => {
                player = Player::new(game_map.get_country_by_index(user_option).clone());
                break;
            },
            4 => {
                player = Player::new(game_map.get_country_by_index(user_option).clone());
                break;
            },
            0 => { exit(0); },
            _ => {
                println!("Invalid choice. Try again");
                continue;
            }
        }
    }

    loop {
        player_check(player, game_map);
    }
}
