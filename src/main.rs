use std::io::{Write, BufReader, BufRead, Error};
use rand::{distributions::Alphanumeric, Rng};
use std::fs::File;
use std::io;
use std::fs::OpenOptions;
use std::ptr::write;
use rocket::{
    config::{Config, Environment, Limits},
    request::Form,
};
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// fn main() -> io::Result<()> {
//     let mut long_url = String::new();
//     let stdin = io::stdin();
//     stdin.read_line(&mut long_url);
//     println!(" input : {}", long_url);
//     let mut n = (long_url.len() as f32 - 0.6* (long_url.len() as f32)).round() as usize;
//     let mut short_url_part = generate_random_string(n);
//     let prefix:String = "https://fernblade.io/".to_owned();
//     let mut short_url = format!("{}{}", prefix,short_url_part);
//     println!("{}",short_url);
//     write_to_file(long_url, short_url);
//     rocket::build().mount("/",routes![index]);
//     // write_to_file("long_url".to_string(), "short_url".to_string());
//     Ok(())
// }
#[post("/", data = "<long_url>")]
fn generate_url(long_url: Form<UserInput>) -> String {
    // let mut long_url = String::new();
    let mut n = (long_url.len() as f32 - 0.6* (long_url.len() as f32)).round() as usize;
    let mut short_url_part = generate_random_string(n);
    let prefix:String = "https://fernblade.io/".to_owned();
    let mut short_url = format!("{}{}", prefix,short_url_part);
    short_url
    // println!("{}",short_url);
    // write_to_file(long_url, short_url);
    // rocket::build().mount("/",routes![index]);
    // // write_to_file("long_url".to_string(), "short_url".to_string());
    // Ok(())
}
fn generate_random_string(num: usize) -> String {
    let mut string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(num)
        .map(char::from)
        .collect();
    string
}

fn write_to_file(long_url: String, short_url: String) -> Result<(), Error> {
    let path="links.txt";
    // let together =
    let mut data_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    write!(data_file, "{}\t{} {}", long_url.trim(),short_url.trim(),"\n")?;
    // writeln!(data_file, "")?;
    // write!(output, "Rust is fun\n")?;

    Ok(())
}

fn main() {
    rocket::custom(config)
        .mount("/", routes![generate_url])
        .launch();
}