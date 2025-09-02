#[derive(Debug, PartialEq, Clone)]
pub enum Role {
    Underboss,
    Caporegime,
    Soldier,
    Associate,
}

impl Role {
    pub fn combat_value(&self) -> u32 {
        match self {
            Role::Underboss => 4,
            Role::Caporegime => 3,
            Role::Soldier => 2,
            Role::Associate => 1,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Member {
    pub role: Role,
    pub age: u32,
}

impl Member {
    pub fn new(role: Role, age: u32) -> Self {
        Member { role, age }
    }

    pub fn get_promotion(&mut self) {
        self.role = match self.role {
            Role::Associate => Role::Soldier,
            Role::Soldier => Role::Caporegime,
            Role::Caporegime => Role::Underboss,
            Role::Underboss => panic!("Cannot be promoted"),
        };
    }
}
