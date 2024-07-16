use crate::models::{Crate, NewCrate};
use crate::repositories::CrateRepository;

use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::DbConn;

#[rocket::get("/get-crates")]
pub async fn get_crates(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>>{
    CrateRepository::find_multiple(&mut db, 50).await
        .map(|crates| json!(crates))
        .map_err(|_err| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::get("/crates/<id>")]
pub async fn view_crate(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>>{
    CrateRepository::find(&mut db, id).await
        .map(|a_crate| json!(a_crate))
        .map_err(|_err| Custom(Status::NotFound, json!("Error")))
}

#[rocket::post("/crates/create", format="json", data="<new_crate>")]
pub async fn create_crate(mut db: Connection<DbConn>, new_crate:Json<NewCrate>) -> Result<Custom<Value>, Custom<Value>>{
    CrateRepository::create(&mut db, new_crate.into_inner()).await
        .map(|a_crate| Custom(Status::Created, json!(a_crate)))
        .map_err(|_err| Custom(Status::InternalServerError, json!("An error occured")))
}


#[rocket::put("/crates/update/<id>", format="json", data="<a_crate>")]
pub async fn update_crate(mut db: Connection<DbConn>, id: i32, a_crate: Json<Crate>) -> Result<Custom<Value>, Custom<Value>>{
    CrateRepository::update(&mut db, id, a_crate.into_inner()).await
    .map(|a_crate| Custom(Status::Ok, json!(a_crate)))
    .map_err(|_err| Custom(Status::InternalServerError, json!("An error occured")))
}

#[rocket::delete("/crates/delete/<id>")]
pub async fn delete_crate(mut db: Connection<DbConn>, id: i32) -> Result<NoContent, Custom<Value>>{
   CrateRepository::delete(&mut db, id).await
    .map(|_| NoContent)
    .map_err(|_err| Custom(Status::InternalServerError, json!("An error occured")))
}