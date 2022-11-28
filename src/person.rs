#[derive(Debug, Clone)]
pub struct Person<'a> {
    id: i64,
    name: &'a str,
    surname: &'a str,
}

impl<'a> Person<'a> {
    pub fn new(id: i64, name: &'a str, surname: &'a str) -> Self {
        Person {
            id: id,
            name: name,
            surname: surname,
        }
    }

    pub fn to_string(&mut self) -> String {
        let string: String = format!("{}: {} {}", self.id, self.name, self.surname);

        string
    }
}
