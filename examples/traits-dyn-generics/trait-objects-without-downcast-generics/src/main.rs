use downcast_rs::{impl_downcast, Downcast};
use rand;
use rand::seq::SliceRandom;
use rand::Rng;
use std::any::{Any, TypeId};
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct Dwarf {}

impl Display for Dwarf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "I may be small but the ladies don't complain!")
    }
}

#[derive(Debug)]
struct Elf {}

#[derive(Debug)]
struct Human {}

#[derive(Debug)]
enum Thing {
    Sword,
    Trinket,
}

trait Enchanter: std::fmt::Debug {
    fn competency(&self) -> f64;

    fn enchant(&self, thing: &mut Thing) {
        let probability_of_success = self.competency();
        let spell_is_successful = rand::thread_rng().gen_bool(probability_of_success); // <1>

        print!("{:?} mutters incoherently. ", self);
        if spell_is_successful {
            println!("The {:?} glows brightly.", thing);
        } else {
            println!(
                "The {:?} fizzes, \
             then turns into a worthless trinket.",
                thing
            );
            *thing = Thing::Trinket {};
        }
    }
}

impl Enchanter for Dwarf {
    fn competency(&self) -> f64 {
        0.5 // <2>
    }
}
impl Enchanter for Elf {
    fn competency(&self) -> f64 {
        0.95 // <3>
    }
}
impl Enchanter for Human {
    fn competency(&self) -> f64 {
        0.8 // <4>
    }
}

/// Holds the log value, stored on the heap.
#[derive(Debug)]
struct LogRecord {
    pub text: String,
}

impl LogRecord {
    fn new() -> Self {
        Self {
            text: String::default(),
        }
    }
}

trait Recordable: Debug {
    fn text(&self) -> String;

    fn set_text(&mut self, _: String);
}

impl Recordable for LogRecord {
    fn set_text(&mut self, value: String) {
        self.text = value;
    }

    fn text(&self) -> String {
        self.text.to_string()
    }
}

fn log_enchanted<'a, T: Any + Debug, U: Recordable>(value: &T, l: &'a mut U) -> &'a U {
    let value_any = value as &dyn Any;

    // Try to convert our value its concrete type
    match value_any.downcast_ref::<Dwarf>() {
        Some(_) => {
            l.set_text("Default text within the recorder".to_string());

            l
        }
        _ => l,
    }
}

fn main() {
    let mut it = Thing::Sword;

    let d = Dwarf {};
    let e = Elf {};
    let h = Human {};

    let party: Vec<&dyn Enchanter> = vec![&d, &h, &e]; // <5>

    // This is a contrived example to try and downcase from a leaked Box, to its concrete type without breaking Safety.
    let d = Dwarf {};
    let mut l = LogRecord::new();
    let r: &LogRecord = log_enchanted(&d, &mut l);
    assert_eq!("Default text within the recorder".to_string(), r.text());

    let spellcaster = party.choose(&mut rand::thread_rng()).unwrap();

    spellcaster.enchant(&mut it);
}
