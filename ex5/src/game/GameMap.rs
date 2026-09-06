use super::Country::Country;

pub struct GameMap {
    countries: Vec<Country>,
}
impl GameMap {
    pub fn new() -> Self {
        let finland = Country::new(String::from("Finland"), 5600000, 900000, vec![], false);
        let sweden = Country::new(String::from("Sweden"), 10000000, 200000, vec![], false);
        let norway = Country::new(String::from("Norway"), 5500000, 100000, vec![], false);
        let denmark = Country::new(String::from("Denmark"), 6000000, 50000, vec![], false);

        let countries = vec![denmark, finland, norway, sweden];
        Self { countries }
    }

    pub fn list_countries(&self) {
        for (i, country) in self.countries.iter().enumerate() {
            println!("{}) {}", i + 1, country.get_name());
        }
    }

    pub fn get_country_by_index(&mut self, i: usize) -> &mut Country {
        &mut self.countries[i]
    }

    pub fn get_countries (&self) -> &Vec<Country> {
        &self.countries
    }

    pub fn set_countries (&mut self, countries: Vec<Country>){
        self.countries = countries;
    }

    pub fn other_countries_turn(&mut self, player_country_name: &String) {
        let mut countries = self.get_countries().clone();
        for country in countries.iter_mut() {
            if !*country.get_is_conquered() && country.get_name() != player_country_name {
                country.add_personel();
            }
        }
        self.set_countries(countries);
}
}