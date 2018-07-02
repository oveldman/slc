use std::thread;
use std::time::Duration;
use database;
use gulden;
use gulden::Market;


pub fn update(){
    loop {
        let current_market: Market = gulden::get_price();
        let current_price: String = current_market.data[0].last.amount.to_string();
        let code: String = current_market.data[0].code.to_string();
        database::client::upload_price_gulden(current_price, code);
        thread::sleep(Duration::from_secs(60));
    }
}