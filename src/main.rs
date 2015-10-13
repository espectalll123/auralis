extern crate hyper;
extern crate rustc_serialize;

// use std::env;
use std::io::Read;

use hyper::Client;
use rustc_serialize::json::Json;

fn main() {    
    println!("When I grow up, I wanna be an AUR helper!");
    // Should implement a let for the RPC URL...
    let mut client = Client::new();
    
    let mut res = client.get("https://aur.archlinux.org/rpc.php?type=search&arg=cowsay")
        .send().unwrap();
    let mut res_body = String::new();
    res.read_to_string(&mut res_body).unwrap();
    let json_data = Json::from_str(&res_body).unwrap();
    let results = json_data.as_object().unwrap().get("results").unwrap();

    for app in results.as_array().unwrap() {
        print!("\n");
        println!("Name: {}", app.as_object().unwrap().get("Name").unwrap());
        println!("Version: {}", app.as_object().unwrap().get("Version").unwrap());
        println!("Description: {}", app.as_object().unwrap().get("Description").unwrap());
        println!("Nº of votes: {}", app.as_object().unwrap().get("NumVotes").unwrap());
    }
}
