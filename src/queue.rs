#[derive(Debug, Clone)]
struct Element<T> {
    data: T,
    next: Option<Box<Element<T>>>
}

impl<T: Clone> Element<T> {
    fn new(data: T) -> Self {
        Element { data: data, next: None }
    }
}

#[derive(Debug, Clone)]
pub struct Queue<T> {
    size: i64,
    beggining: Option<Box<Element<T>>>,
    end: Option<Box<Element<T>>>
}

impl<T: Clone> Queue<T> {
    pub fn new() -> Self {
        Queue { size: 0, beggining: None, end: None }
    }

    pub fn Size(&mut self) -> i64 {
        self.size
    }

    pub fn Add(&mut self, data: T) {
        let newElement: Element<T> = Element::new(data);

        if self.beggining.is_none() {
            self.beggining = Some(Box::new(newElement.clone()));
        } else {
            let end: &mut Box<Element<T>> = self.end.as_mut().unwrap();
            end.next = Some(Box::new(newElement.clone()));
        }

        self.end = Some(Box::new(newElement));
    }

    pub fn Remove(&mut self) -> T {
        if self.beggining.is_none(){
            panic!("Expected begin value");
        }

        let data: T = self.beggining.as_mut().unwrap().data.clone();

        let newBeggining: Option<Box<Element<T>>> = self.beggining.as_mut().unwrap().next.clone();

        self.beggining = newBeggining;

        if self.beggining.is_none() {
            self.end = None;
        }

        data
    }

    pub fn isEmpty(&mut self) -> bool {
        if self.beggining.is_none() {
            return true;
        }

        false
    }
}