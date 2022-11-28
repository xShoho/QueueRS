#[derive(Debug, Clone)]
struct Element<T> {
    data: T,
    next: Option<Box<Element<T>>>,
}

impl<T: Clone> Element<T> {
    fn new(data: T) -> Self {
        Element {
            data: data,
            next: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Queue<T> {
    size: i64,
    beggining: Option<Box<Element<T>>>,
    end: Option<Box<Element<T>>>,
}

impl<T: Clone> Queue<T> {
    pub fn new() -> Self {
        Queue {
            size: 0,
            beggining: None,
            end: None,
        }
    }

    pub fn size(&mut self) -> i64 {
        self.size
    }

    pub fn add(&mut self, data: T) {
        let new_element: Element<T> = Element::new(data);

        if self.beggining.is_none() {
            self.beggining = Some(Box::new(new_element.clone()));
        } else {
            let mut end: Box<Element<T>> = self.end.clone().unwrap();
            end.next = Some(Box::new(new_element.clone()));
        };

        self.end = Some(Box::new(new_element));
    }

    pub fn remove(&mut self) -> T {
        if self.beggining.is_none() {
            panic!("Expected begin value");
        }

        let data: T = self.beggining.as_mut().unwrap().data.clone();

        let new_beggining: Option<Box<Element<T>>> = self.beggining.as_mut().unwrap().next.clone();

        self.beggining = new_beggining;

        if self.beggining.is_none() {
            self.end = None;
        }

        data
    }

    pub fn is_empty(&mut self) -> bool {
        if self.beggining.is_none() {
            return true;
        }

        false
    }
}
