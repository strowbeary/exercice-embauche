use rocket_contrib::json::Json;
use rocket::State;
use core::{Pharmacy};
use db_connector::SQLiteAdapter;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Pagination {
    pub page: u32,
    pub total_nbs_results: u32,
    pub pharmacies: Vec<Pharmacy>
}

#[get("/?<page>")]
pub fn get_pharmacies(sqlite_adapter: State<SQLiteAdapter>, page: Option<u32>) -> Json<Pagination> {
    let page_index = match page {
        None => 0,
        Some(page_index) => page_index
    };
    Json(Pagination {
        page: page_index,
        total_nbs_results: sqlite_adapter.count_all_pharmacies(),
        pharmacies: sqlite_adapter.get_pharmacies(page_index)
    })
}