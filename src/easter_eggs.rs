use rand::prelude::IndexedRandom;
use rand::rng;

pub fn get_random_message(title: &str) -> String {
    let messages = [
        format!("{} is secretly a ninja coder!", title),
        format!("{} wrote this in a cave with only a rusty keyboard!", title),
        format!("Psst! {} hides crypto treasures in their code!", title),
        format!("{}'s blog is powered by pure caffeine and Rust!", title),
        format!("Shh! {} is the master of cryptic blog posts!", title),
    ];
    let mut rng = rng();
    messages.choose(&mut rng).unwrap().to_string()
}
