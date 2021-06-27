#![allow(non_snake_case)]
use std::io::Read;
use reqwest::blocking;// use reqwest::blocking::get to get the dataset

fn download(url: &str) -> Vec<u8>{ // fn to download the data
    let mut response = blocking::get(url).unwrap(); //get synchronous http data from url
    let mut data = Vec::new(); // initialise a new vector
    response.read_to_end(&mut data).unwrap(); //unwrap the data read in response into the data vector

    data // return the data
}
fn main() {
    println!("Hello, world!");
}
