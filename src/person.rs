use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Person<'a> {
    id: i64,
    name: &'a str,
    surname: &'a str,
}

impl<'a> Person<'a> {
    pub fn new(id: i64, name: &'a str, surname: &'a str) -> Self {
        Person { id, name, surname }
    }
}

impl Display for Person<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} {}", self.id, self.name, self.surname)
    }
}
