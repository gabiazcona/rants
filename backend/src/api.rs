use actix_web::{get, post, web, Result, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::dbconnector;


#[derive(Deserialize, Serialize)]
struct QueryCreateRant {
    username: String,
    title: String,
    body: String,
}

/// deserialize `QueryCreateRant` from request's body
#[post("/create_rant")]
async fn create_rant(query: web::Json<QueryCreateRant>) -> Result<String> {
    // make query to DB with Rant
    let mut dbconn = dbconnector::establish_connection();
    let rant = dbconn.create_rant(&query.title, &query.body, &query.username);
    // return ok or error
    Ok(format!("Rant created {}!", rant.title))
}

#[get("/")]
async fn list_rants() -> impl Responder {
    let mut dbconn = dbconnector::establish_connection();
    // retrieve list of rants
    let rants = dbconn.get_rants();
    // response with list of rants
    web::Json(json!(rants))
}