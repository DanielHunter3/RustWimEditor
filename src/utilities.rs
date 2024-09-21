pub struct User {
    is_admin: bool,
    pub name: String
}

impl User {
    pub const fn new(username: String) -> Self {
        User { is_admin: false, name: username }
    }
    
    pub fn is_admin(&self) -> bool {
        self.is_admin
    }
}