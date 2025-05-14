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

    // The pound causes the array to be pretty-printed on multiple lines
    println!("{:#?}", login_event())
}

fn login_event() -> [String; 3] {
    let login_events: [String; 3] = [
        String::from("attempted login"),
        String::from("successful login"),
        String::from("failed login"),
    ];
    login_events
}
