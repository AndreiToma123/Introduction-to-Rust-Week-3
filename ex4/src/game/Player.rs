use super::Country::Country;
use super::GameMap::GameMap;
use std::io;

pub struct Player {
    country: Country,
}

impl Player {
    pub fn new(country:Country) -> Self {
        Self { country }
    }

    pub fn get_country(&mut self) -> &mut Country {
        &mut self.country
    }

    pub fn inspect(&self) {
        println!("An inspection has been completed..");
        println!("Country information:");
        println!("Name: {}", self.country.get_name());
        println!("Population: {}", self.country.get_population());
        println!("Army size: {}", self.country.get_army_size());
    }

    pub fn spy(&self, game_map: &mut GameMap) {
        game_map.list_countries();
    
        let mut choice = String::new();
            io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice = choice.trim();

        let user_choice_spy: usize = match choice.parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Invalid game option. Try again.");
                return;
            },
        };

        if user_choice_spy < 1|| user_choice_spy > 4 {
            println!("Invalid game input. Try again");
            return;
        }
        let enemy = game_map.get_country_by_index(user_choice_spy - 1);

        if enemy.get_name() == self.country.get_name() {
            println!("You can't spy on your own nation!");
        }
        else {
            println!("Espionage successful.");
            println!("Country information:");
            println!("Name: {}", enemy.get_name());
            println!("Population: {}", enemy.get_population());
            println!("Army size: {}", enemy.get_army_size());
        }
    }
}