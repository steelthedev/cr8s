
use crate::models::NewRustacean;
use crate::models::Rustacean;
use crate::repositories::RustaceanRepository;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::response::status::NoContent;
use rocket::serde::json::json;
use rocket::serde::json::Json;
use rocket::serde::json::Value;
use rocket_db_pools::Connection;

use crate::DbConn;



#[rocket::get("/get-rustaceans")]
pub async fn get_rustaceans(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>>{
    RustaceanRepository::find_multiple(&mut db, 50).await
        .map(|rustaceans| json!(rustaceans))
        .map_err(|_err| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::get("/rustaceans/<id>")]
pub async fn view_rustacean(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>>{
    RustaceanRepository::find(&mut db, id).await
        .map(|rustacean| json!(rustacean))
        .map_err(|_err| Custom(Status::NotFound, json!("Error")))
}

#[rocket::post("/rustaceans/create", format="json", data="<new_rustacean>")]
pub async fn create_rustacean(mut db: Connection<DbConn>, new_rustacean: Json<NewRustacean>) -> Result<Custom<Value>, Custom<Value>>{
   RustaceanRepository::create(&mut db, new_rustacean.into_inner()).await
    .map(|rustacean| Custom(Status::Created, json!(rustacean)))
    .map_err(|_err| Custom(Status::InternalServerError, json!("An error occured")))
}

#[rocket::put("/rustaceans/update/<id>", format="json", data="<rustacean>")]
pub async fn update_rustacean(mut db: Connection<DbConn>, id:i32, rustacean: Json<Rustacean>) -> Result<Custom<Value>, Custom<Value>>{
    RustaceanRepository::update(&mut db, id, rustacean.into_inner()).await
    .map(|rustacean| Custom(Status::Ok, json!(rustacean)))
    .map_err(|_err| Custom(Status::InternalServerError, json!("An error occured")))
}

#[rocket::delete("/rustaceans/delete/<id>",)]
pub async fn delete_rustacean(mut db: Connection<DbConn>, id: i32) -> Result<NoContent, Custom<Value>>{
   RustaceanRepository::delete(&mut db, id).await
    .map(|_| NoContent)
    .map_err(|_err| Custom(Status::InternalServerError, json!("An error occured")))
}