use super::show::Show;

#[derive(Debug, Clone)]
pub struct Theater {
    pub id: String,
    pub name: String,
    pub location: String,
    pub shows: Vec<Show>,
}
impl Theater {
    pub fn new(id: String, name: String, location: String) -> Theater {
        Theater {
            id,
            name,
            location,
            shows: Vec::new(),
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_location(&self) -> &str {
        &self.location
    }

    pub fn add_show(&mut self, show: Show) {
        self.shows.push(show);
    }

    pub fn get_shows(&self) -> &Vec<Show> {
        &self.shows
    }
}
