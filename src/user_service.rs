use rjustauth::base::model::AuthUser;
use std::collections::HashMap;

struct UserService {
    user_data: HashMap<String, AuthUser>,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            user_data: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, user: AuthUser) {
        self.user_data.insert(user.uuid.clone(), user);
    }
}
