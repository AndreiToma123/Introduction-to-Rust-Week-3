use super::Country::Country;

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

    // pub fn spy()
}