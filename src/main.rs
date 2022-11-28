mod queue;
mod person;

use rand::{Rng, rngs::ThreadRng};
use std::io;
use person::Person;
use queue::Queue;

fn main() {
    let mut queue: Queue<Person> = Queue::new();

    let surnames = ["Kowalski", "Starosta", "Nowak", "Sikorski", "Pawlak", "Komarski"];
    let names = ["Kamil", "Adrian", "Sebastian", "Sylwester", "Arkadiusz", "Maciek"];

    let mut input_text: String = String::new();
    let mut p: u32 = 0;
    println!("Podaj wskaźnik prawdopodobieństwa (od 0 do 50): ");
    io::stdin().read_line(&mut input_text).expect("Failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => p = i,
        Err(..) => println!("To nie jest liczba dziesiętna: {}", trimmed)
    };

    for i in 0..10 {
        let id1: usize = randomUsize(6); 
        let id2: usize = randomUsize(6); 
        let person: Person = Person::new(i, names[id1], surnames[id2]);
        queue.Add(person);
    }

    let mut i: i64 = 11;
    
    while !queue.isEmpty() {
        if random(101) > p {
            let id1: usize = randomUsize(6); 
            let id2: usize = randomUsize(6); 
            let mut person: Person = Person::new(i, names[id1], surnames[id2]);
            queue.Add(person.clone());
            println!("Wejście klienta {}", person.toString());
            i += 1;
        } else {
            let mut data: Person = queue.Remove();
            println!("Wyjście klienta {}", data.toString());
        }
    }

    println!("Pusta kolejka")
}

fn random(num: u32) -> u32 {
    let mut rng: ThreadRng = rand::thread_rng();

    rng.gen_range(0..num)
}

fn randomUsize(num: usize) -> usize {
    let mut rng: ThreadRng = rand::thread_rng();

    let mut number: usize = rng.gen_range(0..num);

    return number;
}