extern crate serde_json;
extern crate serde_yaml;

#[macro_use]
extern crate serde_derive;

use std::fs::File;


pub type Provider = Vec<ProviderElement>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProviderElement {
    category: Category,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    ctype: String,
    resource: Vec<Resource>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resource {
    name: String,
    url: Url,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Url {
    #[serde(rename = "http")]
    Http,
}

fn main() {
    // let f_str = "/Users/mkmc/projects/rust/j-reader/src/urls.json";
    // let f = File::open(f_str).unwrap();
    // let w: Provider = serde_json::from_reader(f).unwrap();
    // println!("{:?}", w);

    let y_str = "/Users/mkmc/projects/rust/j-reader/src/resource.yml";
    let yf = File::open(y_str).unwrap();
    let w1: Provider = serde_yaml::from_reader(yf).unwrap();
    println!("{:#?}", w1);
}
