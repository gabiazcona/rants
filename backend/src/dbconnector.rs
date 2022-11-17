use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::models::*;
//use diesel::prelude::*;
//use crate::*;

pub struct DBConnector{
    connection:  PgConnection,
}

pub fn establish_connection() -> DBConnector {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    DBConnector{
        connection:  PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }
}

impl DBConnector {    
    pub fn get_rants(&mut self) -> Vec<Rant> {
        use crate::schema::rants::dsl::*;
    
        //let connection = &mut establish_connection();
        let results = rants
            .limit(5)
            .load::<Rant>(&mut self.connection)
            .expect("Error loading posts");
        results
    }
    
    pub fn create_rant(&mut self, title: &str, body: &str, username: &str) -> Rant {
        use crate::schema::rants;
        
        let new_rant = NewRant { title, body, username};
    
        diesel::insert_into(rants::table)
            .values(&new_rant)
            .get_result(&mut self.connection)
            .expect("Error saving new rant")
    }
}