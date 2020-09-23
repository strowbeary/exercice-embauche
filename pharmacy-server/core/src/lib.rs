extern crate serde;

use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct Pharmacy {
    pub id: u32,
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