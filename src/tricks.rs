#![allow(dead_code)]
#![allow(unused_variables)]

use rand::prelude::*;
use rand::Rng;

/** This file is full of little reminders of things you can do with Rust. Check the Polygon part */

pub fn forced_tests() {
    println!("{}", test(8)(7));
    slices()
}

pub fn test (factor: u8) -> Box<Fn(u8) -> u8> {
    Box::new(move |num: u8| factor * num)
}

pub fn slices () {
    let numbers= vec![1, 2, 3, 4, 5, 6, 7];
    //let slice = &numbers[0..=4];
    //let other_slice: Vec<i32> = slice.iter().map(|x| x + 1).collect();
    //let other_numbers: Vec<i32> = numbers.iter().map(|x| x + 1).collect();

    fn mapme (orig: &[i32]) -> Vec<i32> {
        orig.iter().map(|x| x + 1).collect()
    };

    println!("{:?}", mapme(&numbers[0..=4]));
}

pub const ARRAY_NOT_ARRAY: &'static [i32] = &[5];

pub fn array_not() -> &'static [i32] {
    &[5]
}


pub struct Location {
    pub name: &'static str,
    pub governor: &'static str
}


fn locations() -> Vec<Location> {
    let locations = [
        ("Trolland", "Roko"),
    ];

    locations.iter().map(|x| Location::new(*x)).collect()
}

impl Location {
    fn new(name_and_gov: (&'static str, &'static str)) -> Location {
        let (a, b): (&'static str, &'static str) = name_and_gov;
        Location {name: a, governor: b}
    }
}


pub trait Polygon {
    fn perimeter(&self) -> u64;
}

pub struct Square {
    pub side: u64
}

pub struct Triangle {
    pub side: u64
}

impl Polygon for Triangle {
    fn perimeter(&self) -> u64 {
        &self.side * 3
    }
}

impl Polygon for Square {
    fn perimeter(&self) -> u64 {
        &self.side * 4
    }
}

pub fn print_polygons() {
    let mut polygons: Vec<&Polygon> = vec![];
    polygons.push(&Square{side: 4});
    polygons.push(&Triangle{side: 4});

    for pol in polygons {
        println!("{}", pol.perimeter())
    }
}


pub fn randy() -> Vec<i32> {
    let mut rng = thread_rng();
    let y: f64 = rng.gen(); // generates a float between 0 and 1

    let mut nums: Vec<i32> = (1..100).collect();
    nums.shuffle(&mut rng);
    nums
}