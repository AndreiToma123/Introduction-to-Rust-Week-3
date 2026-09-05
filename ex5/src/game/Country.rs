#[derive(Clone)]
pub struct Country {
    name: String,
    population: i64,
    army_size: i64,
    conquered_countries: Vec<String>,
    is_conquered: bool,
}

impl Country {
    pub fn new(
        name: String,
        population: i64,
        army_size: i64,
        conquered_countries: Vec<String>,
        is_conquered: bool,
    ) -> Self {
        Self {
            name: name.to_string(),
            population,
            army_size,
            conquered_countries,
            is_conquered,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_population(&self) -> &i64 {
        &self.population
    }

    pub fn get_army_size(&self) -> &i64 {
        &self.army_size
    }

    pub fn get_is_conquered(&self) ->&bool {
        &self.is_conquered
    }

    pub fn get_conquered_nations(&self) -> &Vec<String> {
        &self.conquered_countries
    }

    pub fn set_population(&mut self, given_population: i64) {
        self.population = given_population;
    }

    pub fn set_army_size(&mut self, given_army_size: i64) {
        self.army_size = given_army_size;
    }

    pub fn set_conquered_nations(&mut self, given_conquered_countries: Vec<String>) {
        self.conquered_countries = given_conquered_countries;
    }
    pub fn set_is_conquered(&mut self, given_is_conquered: bool) {
        self.is_conquered = given_is_conquered;
    }

    pub fn add_personel(&mut self) {
        let new_army = self.army_size + 50000;
        if new_army > self.population {
            self.army_size = self.population;
        }
        else {
            self.army_size = new_army;
        }
    }
}
