#![feature(hash_set_entry)]

use std::{fs, collections::{HashSet, HashMap}, ops::Add, fmt::Display};

struct Person<'a> {
    likes: HashSet<&'a str>,
    dislikes: HashSet<&'a str>
}

fn main() {
    let contents = fs::read_to_string("e_elaborate.in.txt")
        .expect("Something went wrong reading the file");

    let mut lines = contents.lines().into_iter();
    let count = lines.next().unwrap().parse::<usize>().unwrap();

    let mut ingredients = HashMap::<&str, usize>::new();
    let mut persons = Vec::<Person>::with_capacity(count);
    for _ in 0..count {
        let mut likes = HashSet::<&str>::new();
        for ingredient in lines.next().unwrap().split_whitespace().skip(1) {
            register_ingredient(&mut ingredients, ingredient);
            likes.insert(ingredient);
        }
        let mut dislikes = HashSet::<&str>::new();
        for ingredient in lines.next().unwrap().split_whitespace().skip(1) {
            register_ingredient(&mut ingredients, ingredient);
            dislikes.insert(ingredient);
        }

        let person = Person {
            likes,
            dislikes
        };
        persons.push(person);
    }

    /*for ingredient in &ingredients {
        println!("{}", ingredient);
    }*/

    println!("read {} persons and {} ingredients", persons.len(), &ingredients.len());

    let mut pizza = HashSet::<&str>::new();
    loop {
        pizza = get_best_pizza(&pizza, &ingredients, &persons);
        println!("Added {} for pizza score: {}", print_set(&pizza), get_pizza_score(&pizza, &persons));
    }
}

fn print_set<'a>(set: &HashSet<&'a str>) -> String {
    set.iter().map(|s| *s).collect::<Vec<&str>>().join(" ")
}

fn register_ingredient<'a>(ingredients: &mut HashMap<&'a str, usize>, ingredient: &'a str) {
    let entry = ingredients.get_mut(ingredient);
    match entry {
        Some(count) => {
            *count += 1;
        }
        None => {
            ingredients.insert(ingredient, 1);
        },
    };
}

fn get_best_pizza<'a>(pizza: &HashSet<&'a str>, ingredients: &'a HashMap<&str, usize>, persons: &Vec<Person>) -> HashSet<&'a str> {
    let best_pizza = ingredients
        .keys()
        .into_iter()
        .filter_map(|ingredient| {
            if pizza.contains(ingredient) {
                return None;
            }
            let mut candidate = pizza.clone();
            candidate.insert(ingredient);
            Some(candidate)
        })
        .max_by_key(|candidate| {
            get_pizza_score(&candidate, persons)
        });
    best_pizza.unwrap().to_owned()
}

fn get_pizza_score(pizza: &HashSet<&str>, persons: &Vec<Person>) -> usize {
    let mut score: usize = 0;
    for person in persons {
        if person.likes.is_subset(pizza) && person.dislikes.is_disjoint(pizza) {
            score += 1;
        }
    }

    score
}