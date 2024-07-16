mod models;
mod schema;
mod repositories;
mod rocket_routes;

use rocket_db_pools::Database;


#[derive(Database)]
#[database("postgres")]
struct DbConn(rocket_db_pools::diesel::PgPool);


#[rocket::main]
async fn main() {
   let _ = rocket::build()
        .mount("/", rocket::routes![
           rocket_routes::get_rustaceans
        ])
        .attach(DbConn::init())
        .launch()
        .await;
}
