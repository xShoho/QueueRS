mod person;
mod queue;

use person::Person;
use queue::Queue;
use rand::{rngs::ThreadRng, Rng};
use std::io;

fn main() {
    let surnames = [
        "Kowalski", "Starosta", "Nowak", "Sikorski", "Pawlak", "Komarski",
    ];
    let names = [
        "Kamil",
        "Adrian",
        "Sebastian",
        "Sylwester",
        "Arkadiusz",
        "Maciek",
    ];

    let p: u32 = get_probability_from_user();
    let mut queue = initialize_queue(names, surnames);

    println!("#### Zinicjalizowano Kolejkę ####");

    let mut id: i64 = 10;
    let num: usize = 6;

    while !queue.is_empty() {
        if random(101) > p {
            let id1: usize = random_usize(num);
            let id2: usize = random_usize(num);
            let data: Person = Person::new(id, names[id1], surnames[id2]);
            println!(
                "\nElementów w kolejce: {}\nWejście klienta {}",
                queue.size() + 1,
                data
            );
            queue.add(data);
            id += 1;
        } else {
            let data: Person = queue.remove();
            println!(
                "\nElementow w kolejce: {}\nWyjście klienta {}",
                queue.size() + 1,
                data
            );
        }
    }

    println!("Pusta kolejka")
}

fn random(num: u32) -> u32 {
    let mut rng: ThreadRng = rand::thread_rng();

    rng.gen_range(0..num)
}

fn random_usize(num: usize) -> usize {
    let mut rng: ThreadRng = rand::thread_rng();

    let number: usize = rng.gen_range(0..num);

    number
}

fn initialize_queue<'a>(names: [&'a str; 6], surnames: [&'a str; 6]) -> Queue<Person<'a>> {
    println!("\n#### Inicjalizacja Elementów w kolejce ####");

    let mut queue = Queue::new();
    for i in 0..10 {
        let id1 = random_usize(6);
        let id2 = random_usize(6);
        let name = names[id1];
        let surname = surnames[id2];
        let person = Person::new(i, name, surname);
        queue.add(person);
    }

    queue
}

fn get_probability_from_user() -> u32 {
    let mut input_text = String::new();
    let mut return_value: u32 = 0;
    println!("Podaj wskaźnik prawdopodobieństwa (od 0 do 50): ");
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => return_value = i,
        Err(..) => println!(
            "To nie jest liczba dziesiętna lub mniejsza niż zero: {}",
            trimmed
        ),
    };

    if return_value > 50 {
        println!("Liczba jest większa niż 50: {}", return_value);
        return_value = 0;
    }

    return_value
}
