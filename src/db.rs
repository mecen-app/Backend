use std::env;

use mongodb::{error::Error, options::ClientOptions, sync::Client, sync::Database};

fn database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL not set").to_string()
}

fn database_name() -> String {
    "tests".to_string()
}

pub(crate) fn connect() -> Result<Database, Error> {
    let _client_options = ClientOptions::parse(database_url())?;
    let client = Client::with_uri_str(database_url())?;
    let database = client.database(&*database_name());
    Ok(database)
}
