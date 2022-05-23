use std::env;

use mongodb::{options::ClientOptions, sync::Client, sync::Database, error::Error};

fn database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub(crate) fn connect() -> Result<Database, Error> {
    let _client_options = ClientOptions::parse(
        database_url(),
    )?;
    let client = Client::with_uri_str(database_url())?;
    let database = client.database("mecen");
    Ok(database)
}
