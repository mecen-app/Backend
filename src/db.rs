use std::env;

use mongodb::{error::Error, options::ClientOptions, sync::Client, sync::Database};

fn database_url() -> String {
    env!("DATABASE_URL").to_string()
}

fn database_name() -> String {
    match env::var("ENVIRONMENT") {
        Ok(val) => val,
        Err(e) => {
            dbg!(e);
            "tests".to_string()
        }
    }
}

pub(crate) fn connect() -> Result<Database, Error> {
    let _client_options = ClientOptions::parse(database_url())?;
    let client = Client::with_uri_str(database_url())?;
    let database = client.database(&*database_name());
    Ok(database)
}
