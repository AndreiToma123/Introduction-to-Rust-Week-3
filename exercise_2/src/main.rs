mod game;
use game::Country::Country;
use game::Player::Player;
use std::io;

pub fn player_check(player:Player) {
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
}

fn main() {

    let finland = Country::new(String::from("Finland"), 5600000, 900000, vec![], false);
    let sweden = Country::new(String::from("Sweden"), 10000000, 200000, vec![], false);
    let norway = Country::new(String::from("Norway"), 5500000, 100000, vec![], false);
    let denmark = Country::new(String::from("Denmark"), 6000000, 50000, vec![], false);

    println!("| 1) Finland | 2) Sweden | 3) Norway | 4) Denmark |");

    let player: Player;

    loop {
        println!("Choose your country: ");

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

        match user_option {
            1 => {
                player = Player::new(finland);
                player_check(player);
                break;
            },
            2 => {
                player = Player::new(sweden);
                player_check(player);
                break;                
            },
            3 => {
                player = Player::new(norway);
                player_check(player);
                break;
            },
            4 => {
                player = Player::new(denmark);
                player_check(player);
                break;
            },
            _ => {
                println!("Invalid choice. Try again");
                continue;
            }
        }
    }
}
