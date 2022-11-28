#[derive(Debug, Clone)]
struct Element<T> {
    data: T,
    next: Option<Box<Element<T>>>,
}

impl<T: Clone> Element<T> {
    fn new(data: T) -> Self {
        Element { data, next: None }
    }
}

#[derive(Debug, Clone)]
pub struct Queue<T> {
    size: i64,
    beggining: Option<Box<Element<T>>>,
}

impl<T: Clone> Queue<T> {
    pub fn new() -> Self {
        Queue {
            size: 0,
            beggining: None,
        }
    }

    pub fn size(&mut self) -> i64 {
        self.size
    }

    pub fn add(&mut self, data: T) {
        let new_element = Some(Box::new(Element::new(data)));

        let mut current = match self.beggining.as_mut() {
            Some(element) => element,
            None => {
                self.beggining = new_element;
                return;
            }
        };

        while current.next.is_some() {
            current = current.next.as_mut().unwrap();
        }

        current.next = new_element;
        self.size += 1;
    }

    pub fn remove(&mut self) -> T {
        let data = self.beggining.as_mut().unwrap().data.clone();

        let new_beggining = match self.beggining.as_mut() {
            Some(element) => element.next.take(),
            None => None,
        };

        self.beggining = new_beggining;
        self.size -= 1;

        data
    }

    pub fn is_empty(&mut self) -> bool {
        if self.beggining.is_none() {
            return true;
        }

        false
    }
}
