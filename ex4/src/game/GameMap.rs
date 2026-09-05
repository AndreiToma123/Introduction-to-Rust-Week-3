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
}