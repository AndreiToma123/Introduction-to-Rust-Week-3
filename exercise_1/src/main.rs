mod game;
use game::Country::Country;
use std::{any::type_name, io};

fn main() {

    let finland = Country::new(String::from("Finland"), 5600000, 900000, vec![], false);
    let sweden = Country::new(String::from("Sweden"), 10000000, 200000, vec![], false);
    let norway = Country::new(String::from("Norway"), 5500000, 100000, vec![], false);
    let denmark = Country::new(String::from("Denmark"), 6000000, 50000, vec![], false);

    println!("| 1) Finland | 2) Sweden | 3) Norway | 4) Denmark |");

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
                println!("Country: {}", finland.get_name());
                println!("Population: {}", finland.get_population());
                println!("Army size: {}", finland.get_army_size());
                break;
            },
            2 => {
                println!("Country: {}", sweden.get_name());
                println!("Population: {}", sweden.get_population());
                println!("Army size: {}", sweden.get_army_size());
                break;                
            },
            3 => {
                println!("Country: {}", norway.get_name());
                println!("Population: {}", norway.get_population());
                println!("Army size: {}", norway.get_army_size());
                break;
            },
            4 => {
                println!("Country: {}", denmark.get_name());
                println!("Population: {}", denmark.get_population());
                println!("Army size: {}", denmark.get_army_size());
                break;
            },
            _ => {
                println!("Invalid choice. Try again");
                continue;
            }
        }
    }
}
