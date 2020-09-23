extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;

use rusqlite::{params};
use rusqlite::NO_PARAMS;
use r2d2_sqlite::SqliteConnectionManager;

use r2d2::Pool;
use core::Pharmacy;
use std::iter::FromIterator;

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
                cp text not null,
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

    pub fn create_etablissement(&self, etablissement: Pharmacy) {
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
                &etablissement.nofinesset.to_string(),
                &etablissement.nofinessej.to_string(),
                &etablissement.rs.to_string(),
                &etablissement.rslongue,
                &etablissement.complrs,
                &etablissement.numvoie,
                &etablissement.compvoie,
                &etablissement.typvoie,
                &etablissement.voie.to_string(),
                &etablissement.lieuditbp,
                &etablissement.departement,
                &etablissement.libdepartement.to_string(),
                &etablissement.cp.to_string(),
                &etablissement.commune.to_string(),
                &etablissement.telephone,
                &etablissement.telecopie,
                &etablissement.dateouv.to_string(),
                &etablissement.dateautor.to_string(),
                &etablissement.datemaj.to_string(),
                &etablissement.wgs84.to_string(),
                &etablissement.lat,
                &etablissement.lng
            ],
        ).expect("Booking creation in DB failed");
    }

    pub fn get_pharmacies(&self, page: u32) -> Vec<Pharmacy> {
        let page_size = 10;
        let conn = self.pool.get().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM pharmacies limit ?1 offset ?2").unwrap();
        Vec::from_iter(
            stmt
                .query_map(params![page_size, page_size * page], |row| {
                    Ok(Pharmacy {
                        id: row.get(0).unwrap(),
                        nofinesset: row.get(1).unwrap(),
                        nofinessej: row.get(2).unwrap(),
                        rs: row.get(3).unwrap(),
                        rslongue: row.get(4).unwrap(),
                        complrs: row.get(5).unwrap(),
                        numvoie: row.get(6).unwrap(),
                        compvoie: row.get(7).unwrap(),
                        typvoie: row.get(8).unwrap(),
                        voie: row.get(9).unwrap(),
                        lieuditbp: row.get(10).unwrap(),
                        departement: row.get(11).unwrap(),
                        libdepartement: row.get(12).unwrap(),
                        cp: row.get(13).unwrap(),
                        commune: row.get(14).unwrap(),
                        telephone: row.get(15).unwrap(),
                        telecopie: row.get(16).unwrap(),
                        dateouv: row.get(17).unwrap(),
                        dateautor: row.get(18).unwrap(),
                        datemaj: row.get(19).unwrap(),
                        wgs84: row.get(20).unwrap(),
                        lat: row.get(21).unwrap(),
                        lng: row.get(22).unwrap()
                    })
                })
                .unwrap()
                .map(|row| row.unwrap())
        )
    }
    pub fn count_all_pharmacies(&self) -> u32 {
        let conn = self.pool.get().unwrap();
        conn.query_row(
            "SELECT count(*) FROM pharmacies",
            NO_PARAMS,
            |r| {
                Ok(r.get_unwrap(0))
            }
        ).unwrap()
    }
}

unsafe impl Send for SQLiteAdapter {}

unsafe impl Sync for SQLiteAdapter {}
