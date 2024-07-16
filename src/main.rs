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
           rocket_routes::rustaceans::get_rustaceans,
           rocket_routes::rustaceans::view_rustacean,
           rocket_routes::rustaceans::update_rustacean,
           rocket_routes::rustaceans::create_rustacean,
           rocket_routes::rustaceans::delete_rustacean,

           rocket_routes::crates::get_crates,
           rocket_routes::crates::view_crate,
           rocket_routes::crates::update_crate,
           rocket_routes::crates::create_crate,
           rocket_routes::crates::delete_crate,
           
        ])
        .attach(DbConn::init())
        .launch()
        .await;
}
