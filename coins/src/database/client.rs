use postgres::{Connection, TlsMode};
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct CryptoCoins {
    pub gulden: Vec<Gulden>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gulden {
    pub price: String,
    pub code: String,
    pub datetime: String,
}

fn connect() -> Connection {
    let connection: Connection = Connection::connect("postgres://ictlab:LuckyStrike80@localhost:5432/coins", TlsMode::None).unwrap();
    connection
}

pub fn update() {
    let connection = connect();
        connection.execute("CREATE TABLE IF NOT EXISTS gulden (
                    id              SERIAL PRIMARY KEY,
                    price           VARCHAR NOT NULL,
                    code            VARCHAR NOT NULL,
                    upload_date     VARCHAR NOT NULL
                  )", &[]).unwrap();
}

pub fn upload_price_gulden(price: String, code: String){
    let connection = connect();
    let local_time_now: DateTime<Local> = Local::now();
        connection.execute("INSERT INTO gulden (price, code, upload_date) VALUES ($1, $2, $3)",
                 &[&price.to_string(), &code.to_string(), &local_time_now.to_string()]).unwrap();
}

pub fn get_all_price_gulden() -> CryptoCoins {
    let connection = connect();
    let mut all_prices_gulden: Vec<Gulden> = Vec::new();
    for row in &connection.query("SELECT price, code, upload_date FROM gulden", &[]).unwrap() {
        let current_gulden = Gulden {
            price: row.get(0),
            code: row.get(1),
            datetime: row.get(2),
        };
        all_prices_gulden.push(current_gulden);
    };

    let new_crypto = CryptoCoins {
        gulden: all_prices_gulden,
    };

   new_crypto
}