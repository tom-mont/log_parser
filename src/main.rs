use fake::faker::name::raw::*;
use fake::locales::EN;
use fake::rand::SeedableRng;
use fake::rand::rngs::StdRng;
use fake::{Dummy, Fake, Faker};

fn main() {
    println!("Hello, world!");
    let name: String = Name(EN).fake();
    let event: String = String::from("attempted login");
    println!("name: {}\nevent: {}", name, event);
}
