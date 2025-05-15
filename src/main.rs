use fake::faker::name::raw::*;
use fake::locales::EN;
use fake::rand::SeedableRng;
use fake::rand::rngs::StdRng;
use fake::{Dummy, Fake, Faker};
use rand::prelude::*;

fn main() {
    println!("Hello, world!");
    let name: String = Name(EN).fake();
    let event: String = String::from("attempted login");
    println!("name: {}\nevent: {}", name, event);

    // The pound causes the array to be pretty-printed on multiple lines
    println!("{:#?}, {}", login_event(), generate_user_id());
    println!("{:#?}, {}", login_event(), generate_user_id());
    println!("{:#?}, {}", login_event(), generate_user_id());
    println!("{:#?}, {}", login_event(), generate_user_id());
    println!("{:#?}, {}", login_event(), generate_user_id());
    println!("{:#?}, {}", login_event(), generate_user_id());
    println!("{:#?}, {}", login_event(), generate_user_id());
    println!("{:#?}, {}", login_event(), generate_user_id());
    println!("{:#?}, {}", login_event(), generate_user_id());
}

fn login_event() -> String {
    let login_events: [String; 3] = [
        String::from("attempted login"),
        String::from("successful login"),
        String::from("failed login"),
    ];

    let mut rng = rand::rng();
    let random_float: f32 = rng.random::<f32>();
    let login_event: String = if random_float <= 0.33 {
        String::from("attempted login")
    } else if (0.33 < random_float) && (random_float <= 0.66) {
        String::from("successful login")
    } else {
        String::from("failed login")
    };
    login_event
}

fn generate_user_id() -> u8 {
    let mut rng: ThreadRng = rand::rng();
    // Select an integer from 1 (inclusive) to 10 (inclusive)
    let user_id: u8 = rng.random_range(1..=10);
    user_id
}
