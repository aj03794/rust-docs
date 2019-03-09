
// Need pub keyword to be able to use in main functions
// Using super rather than an abolsute path starting with crate
// is that using super may make it easier to update your code to have
// a different module hierarchy
// For example if we wrapped the in another module, the super would still work

// #[derive(Debug)]

mod sound {
    pub mod instrument {
        pub fn clarinet() {
            println!("hello from clarinet");
            super::breathe_in();
        }
    }
    fn breathe_in() {
        println!("hello from breathe_in");
    }
}

mod plant {
    pub struct Vegetable {
        // public name field but private id field
        pub name: String,
        id: i32
    }
    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1
            }
        }
    }
}

mod menu {
    pub enum Appetizer {
        Soup,
        Salad
    }
}

fn main() {
    // absolute path
    crate::sound::instrument::clarinet();

    // relative path
    sound::instrument::clarinet();

    let mut v = plant::Vegetable::new("squash");

    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name);

    // The next line won't compile if we uncomment it:
    // We can't access id in this scope because id is private
    // println!("The ID is {}", v.id);

    let order1 = menu::Appetizer::Soup;
    let order2 = menu::Appetizer::Salad;
}
