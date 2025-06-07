pub struct Movie {
    pub id: String,
    pub title: String,
    pub name: String,
    pub release_year: i32,
}

impl Movie {
    pub fn new(id: String, title: String, name: String, release_year: i32) -> Movie {
        Movie {
            id,
            title,
            name,
            release_year,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn get_title(&self) -> &str {
        &self.title
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_release_year(&self) -> i32 {
        self.release_year
    }
}
