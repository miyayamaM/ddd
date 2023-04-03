use crate::value_objects::user_id::UserId;

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub name: String,
}

impl User {
    pub fn new(id: UserId, name: String) -> Self {
        if name.len() < 3 {
            panic!("name has to be more than 3 characters.")
        };
        Self { id, name }
    }

    pub fn id(&self) -> UserId {
        self.id.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn change_user_name(&mut self, name: String) {
        if name.len() < 3 {
            panic!("name has to be more than 3 characters.")
        };
        self.name = name;
    }
}
