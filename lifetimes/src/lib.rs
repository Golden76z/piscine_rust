#[derive(Debug, Clone)]
pub struct Person {
    pub name: &'static str,
    pub age: u8,
}

impl<'a> Person {
    pub fn new(name: &'static str) -> Person {
        Person {
            name: &name,
            age: 0,
        }
    }
}
