extern crate serde;

mod database_adapter;

use std::error::Error;
use serde::Deserialize;
use std::{process, env};
use std::fs::File;
use std::ffi::OsString;
use crate::database_adapter::SQLiteAdapter;

#[derive(Debug, Deserialize)]
pub struct Pharmacy {
    pub nofinesset: u32,
    pub nofinessej: u32,
    pub rs: String,
    pub rslongue: Option<String>,
    pub complrs: Option<String>,
    pub numvoie: Option<u32>,
    pub compvoie: Option<String>,
    pub typvoie: Option<String>,
    pub voie: String,
    pub lieuditbp: Option<String>,
    pub departement: u32,
    pub libdepartement: String,
    pub cp: u32,
    pub commune: String,
    pub telephone: Option<u32>,
    pub telecopie: Option<u32>,
    pub dateouv: String,
    pub dateautor: String,
    pub datemaj: String,
    pub wgs84: String,
    pub lat: f64,
    pub lng: f64,
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn parse() -> Result<(), Box<dyn Error>> {
    let db_adapter = SQLiteAdapter::new();
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(file);
    for result in rdr.deserialize() {
        let record: Pharmacy = result?; //possible thanks to serde (deserialization lib)
        db_adapter.create_pharmacy(record);
    }
    println!("Done.");
    Ok(())
}

fn main() {
    if let Err(err) = parse() {
        println!("error : {}", err);
        process::exit(1);
    }
}