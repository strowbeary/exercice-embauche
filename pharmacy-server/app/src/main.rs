use rest_api::start_api;
use db_connector::SQLiteAdapter;


fn main() {
    start_api(SQLiteAdapter::new());
}
