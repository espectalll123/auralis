extern crate hyper;
extern crate rustc_serialize;

// use std::env;
use std::io::Read;

use hyper::Client;
use rustc_serialize::json::Json;

fn main() {    
    println!("When I grow up, I wanna be an AUR helper!");
    
    /*
    println!("Meanwhile, here are your arguments <3");
    let mut count = 0;
    for argument in env::args() {
        count += 1;
        println!("{}· {}", count, argument);
    }
     */

    // Should implement a let for the RPC URL...
    let mut client = Client::new();
    
    let mut res = client.get("https://aur.archlinux.org/rpc.php?type=search&arg=cowsay")
        .send().unwrap();
    let mut res_body = String::new();
    res.read_to_string(&mut res_body)
        .unwrap();
    let json_data = Json::from_str(&res_body)
        .unwrap();
    println!("{}", json_data.pretty());
    /*
    let json_obj = json_data
        .as_object()
        .unwrap();
    let results = json_obj.get("results")
        .unwrap();
     */
    
    // I *really* have no idea about Rust...
}
