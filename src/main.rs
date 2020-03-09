#[macro_use]
extern crate serde_derive;

mod settings;
use settings::Settings;

fn main() {
    let res = Settings::new();
    match res {
        Ok(s) => {
            println!("{:?}", s);
        },
        Err(err) => {
            println!("Something went wrong while reading the configuration. Aborting. : {:?}", err);
            std::process::exit(1);
        }
    }
}
