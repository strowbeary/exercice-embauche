extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;
extern crate core;

use rusqlite::{params};
use rusqlite::NO_PARAMS;
use r2d2_sqlite::SqliteConnectionManager;

use r2d2::Pool;
use crate::Pharmacy;

pub struct SQLiteAdapter {
    pool: Pool<SqliteConnectionManager>
}

impl SQLiteAdapter {
    pub fn new() -> SQLiteAdapter {
        let manager = SqliteConnectionManager::file("pharmacies.db");
        let pool = r2d2::Pool::new(manager).unwrap();
        let conn = pool.get().unwrap();

        conn.execute(
            "create table if not exists pharmacies (
                id integer primary key,
                nofinesset integer not null,
                nofinessej integer not null,
                rs text not null,
                rslongue text,
                complrs text,
                numvoie integer,
                compvoie text,
                typvoie text,
                voie text not null,
                lieuditbp text,
                departement integer not null,
                libdepartement text not null,
                cp integer not null,
                commune text not null,
                telephone integer,
                telecopie integer,
                dateouv datetime not null,
                dateautor datetime not null,
                datemaj datetime not null,
                wgs84 text not null,
                lat real not null,
                lng real not null
            )",
            NO_PARAMS,
        ).expect("Creation of the table 'pharmacies' failed");

        SQLiteAdapter {
            pool
        }
    }

    pub fn create_pharmacy(&self, pharmacy: Pharmacy) {
        let conn = self.pool.get().unwrap();
        conn.execute(
            "insert into pharmacies (
                nofinesset,
                nofinessej,
                rs,
                rslongue,
                complrs,
                numvoie,
                compvoie,
                typvoie,
                voie,
                lieuditbp,
                departement,
                libdepartement,
                cp,
                commune,
                telephone,
                telecopie,
                dateouv,
                dateautor,
                datemaj,
                wgs84,
                lat,
                lng
            )
            values
            (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22)",
            params![
                &pharmacy.nofinesset,
                &pharmacy.nofinessej,
                &pharmacy.rs.to_string(),
                &pharmacy.rslongue,
                &pharmacy.complrs,
                &pharmacy.numvoie,
                &pharmacy.compvoie,
                &pharmacy.typvoie,
                &pharmacy.voie.to_string(),
                &pharmacy.lieuditbp,
                &pharmacy.departement,
                &pharmacy.libdepartement.to_string(),
                &pharmacy.cp,
                &pharmacy.commune.to_string(),
                &pharmacy.telephone,
                &pharmacy.telecopie,
                &pharmacy.dateouv.to_string(),
                &pharmacy.dateautor.to_string(),
                &pharmacy.datemaj.to_string(),
                &pharmacy.wgs84.to_string(),
                &pharmacy.lat,
                &pharmacy.lng
            ],
        ).expect("Booking creation in DB failed");
    }
}

unsafe impl Send for SQLiteAdapter {}

unsafe impl Sync for SQLiteAdapter {}
