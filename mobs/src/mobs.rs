pub mod boss;
pub mod members;

pub use boss::Boss;
pub use members::{Member, Role};

use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: HashMap<String, Member>,
    pub cities: HashSet<String>,
    pub wealth: u64,
}

impl Mob {
    pub fn new(name: String, boss: Boss) -> Self {
        Mob {
            name,
            boss,
            members: HashMap::new(),
            cities: HashSet::new(),
            wealth: 0,
        }
    }

    pub fn recruit(&mut self, member_info: (&str, u32)) {
        let (name, age) = member_info;
        let member = Member::new(Role::Associate, age);
        self.members.insert(name.to_string(), member);
    }

    pub fn combat_power(&self) -> u32 {
        self.members
            .values()
            .map(|member| member.role.combat_value())
            .sum()
    }

    pub fn remove_youngest_members(&mut self) {
        if self.members.is_empty() {
            return;
        }
        let min_age = self.members.values().map(|m| m.age).min().unwrap();
        self.members.retain(|_, member| member.age != min_age);
    }

    pub fn attack(&mut self, target: &mut Mob) {
        let self_power = self.combat_power();
        let target_power = target.combat_power();
        if target_power < self_power {
            target.remove_youngest_members();
            if target.members.is_empty() {
                self.wealth += target.wealth;
                target.wealth = 0;
                for city in &target.cities {
                    self.cities.insert(city.clone());
                }
                target.cities.clear();
            }
        } else {
            self.remove_youngest_members();
            if self.members.is_empty() {
                target.wealth += self.wealth;
                self.wealth = 0;
                for city in &self.cities {
                    target.cities.insert(city.clone());
                }
                self.cities.clear();
            }
        }
    }

    pub fn steal(&mut self, target: &mut Mob, amount: u64) {
        let stolen_amount = amount.min(target.wealth);
        self.wealth += stolen_amount;
        target.wealth -= stolen_amount;
    }

    pub fn conquer_city(&mut self, other_mobs: &[&Mob], city_name: String) {
        let city_taken = other_mobs.iter().any(|mob| mob.cities.contains(&city_name));
        if !city_taken {
            self.cities.insert(city_name);
        }
    }
}
