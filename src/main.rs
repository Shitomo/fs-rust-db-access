use postgres::{Client};
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_ca_file("root.crt").unwrap();
    let connector = MakeTlsConnector::new(builder.build());
    let mut client = Client::connect(&database_url, connector).unwrap();
    client.execute("CREATE TABLE sample_table (
                    id              SERIAL PRIMARY KEY,
                    value            INTEGER
    )", &[]).unwrap();
}
