#![allow(non_snake_case)]
use std::io::{Cursor, Read}; // the reader used in downloading the data
use reqwest::blocking;// use reqwest::blocking::get to get the dataset
use zip::read::ZipArchive;


fn download(url: &str) -> Vec<u8>{ // fn to download the data
    let mut response = blocking::get(url).unwrap(); //get synchronous http data from url
    let mut data = Vec::new(); // initialise a new vector
    response.read_to_end(&mut data).unwrap(); //unwrap the data read in response into the data vector

    data // return the data
}

fn unzip(zip_file: Vec<u8>) -> String{
    // cursor wraps an in-memory buffer to give it a seek implementation. since that is where our downloaded file is we use cursor to unwrap our zip
    let mut archive = ZipArchive::new(Cursor::new(zip_file)).unwrap(); // create an object to read the zip file
    let mut file = archive.by_name("SMSSpamCollection").unwrap(); //SMSSpamCollection is the name of the required file within the zip
    
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap(); // unwrapping the file into a string

    data
}
fn main() {
    let zipped = download("https://archive.ics.uci.edu/ml/machine-learning-databases/00228/smsspamcollection.zip");
    let raw_data = unzip(zipped);

    for line in raw_data.lines().take(3) {
        println!("{}", line); // sanity check to see if everything worked
    }

}
