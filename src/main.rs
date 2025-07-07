use fake::faker::name::raw::*;
use fake::locales::EN;
//use fake::rand::SeedableRng;
//use fake::rand::rngs::StdRng;
use fake::{Dummy, Fake, Faker};
use rand::prelude::*;

fn main() {
    println!("Hello, world!");
    let name: String = Name(EN).fake();
    let event: String = String::from("attempted login");
    println!("name: {}\nevent: {}", name, event);

    // The pound causes the array to be pretty-printed on multiple lines
    let mut i: u8 = 0;
    let mut last_user_id: u8 = 0;
    let mut last_login_event: String = String::from("null");
    let mut current_user_id: u8;
    let mut current_login_event: String;
    while i < 10 {
        current_user_id = generate_user_id();
        current_login_event = login_event();
        println!("{:#?}, {}", &current_login_event, &current_user_id);

        if (last_user_id == current_user_id)
            && (current_login_event == "failed login")
            && (current_login_event == last_login_event)
        {
            println!(
                "Two unsuccessful login attempts in a row by user ID: {}",
                &current_user_id
            )
        } else if last_user_id == current_user_id {
            println!("Last user ID matches current user ID: {}", &current_user_id);
        }
        i += 1;
        last_user_id = current_user_id;
        last_login_event = current_login_event;
    }
}

fn login_event() -> String {
    // commenting out -> implemented in simpler way below
    //    let login_events: [String; 3] = [
    //        String::from("attempted login"),
    //        String::from("successful login"),
    //        String::from("failed login"),
    //    ];

    let mut rng = rand::rng();
    let random_float: f32 = rng.random::<f32>();
    let login_event: String = if random_float <= 0.2 {
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
