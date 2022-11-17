use actix_web::{get, post, web, Result, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{dbconnector, dbconnector::DBConnector};


#[derive(Deserialize, Serialize)]
struct ApiRant {
    username: String,
    title: String,
    body: String,
}

/// deserialize `QueryCreateRant` from request's body
#[post("/create_rant")]
async fn create_rant(query: web::Json<ApiRant>) -> Result<String> {
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
    let rants = get_list_rants(&mut dbconn);
    // format rants?
    // response with list of rants
    web::Json(json!(rants))
}

fn get_list_rants(dbconn: &mut DBConnector) -> Vec<ApiRant> {
    
    let rants = dbconn.get_rants();
    rants.iter().map(|r| 
            ApiRant {
                username: r.username.clone(),
                title: r.title.clone(),
                body: r.body.clone(),
            }
    ).collect()
}
