mod game;
use game::Country::Country;
use game::Player::Player;
use std::io;
use std::process::exit;

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

    println!("| 1) Spy on a country | 0) Exit program |");
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice = choice.trim();

    let mut user_choice = match choice.parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Invalid option. Try again.");
            continue;
        },
    }
    match user_choice {
        1 => {
            // spy();
        },
        2 => exit(0),
        _ => {
            println!("Invalid input. Try again.");
            continue;
        }
    }

}

fn main() {

    let finland = Country::new(String::from("Finland"), 5600000, 900000, vec![], false);
    let sweden = Country::new(String::from("Sweden"), 10000000, 200000, vec![], false);
    let norway = Country::new(String::from("Norway"), 5500000, 100000, vec![], false);
    let denmark = Country::new(String::from("Denmark"), 6000000, 50000, vec![], false);

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

        match user_option {
            1 => {
                player = Player::new(finland.clone());
                player_check(player);
            },
            2 => {
                player = Player::new(sweden.clone());
                player_check(player);
            },
            3 => {
                player = Player::new(norway.clone());
                player_check(player);
            },
            4 => {
                player = Player::new(denmark.clone());
                player_check(player);
            },
            0 => exit(0),
            _ => {
                println!("Invalid choice. Try again");
                continue;
            }
        }
    }
}
