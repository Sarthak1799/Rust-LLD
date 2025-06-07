#[derive(Debug, Clone)]
pub struct Customer {
    pub id: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub license_number: String,
}

impl Customer {
    pub fn new(
        id: String,
        name: String,
        email: String,
        phone: String,
        license_number: String,
    ) -> Self {
        Self {
            id,
            name,
            email,
            phone,
            license_number,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn get_phone(&self) -> &str {
        &self.phone
    }

    pub fn get_license_number(&self) -> &str {
        &self.license_number
    }
}
