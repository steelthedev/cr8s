use diesel::QueryResult;
use diesel_async::AsyncPgConnection;
use crate::{models::Rustacean, schema::rustaceans};
pub struct RustaceanRepository;


impl RustaceanRepository {
    pub async fn find(c: &AsyncPgConnection,id: i32) -> QueryResult<Rustacean>{
        rustaceans::table.find(id).get_result(c).await;
    }
}