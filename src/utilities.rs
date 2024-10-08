pub struct User {
    is_admin: bool,
    name: String
}

impl User {
    pub const fn new(username: String) -> Self {
        User { is_admin: false, name: username }
    }
    
    pub fn is_admin(&self) -> bool {
        self.is_admin
    }

    #[allow(dead_code)]
    pub fn set_admin(&mut self, is_admin: bool) {
        self.is_admin = is_admin;
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}