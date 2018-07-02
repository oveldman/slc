use serde_json;
use serde_json::Error;
use reader;
use request;
use std::str;

const GULDEN_FILE: &str = "gulden_request";

#[derive(Serialize, Deserialize, Debug)]
pub struct Market {
    pub data: Vec<Round>,
    meta: Meta,
    status: u8
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Round {
    pub code: String,
    pub last: Price,
    volume: Price,
    low: Price,
    high: Price,
    buy: Price,
    sell: Price,
    is_active: bool,
    resource: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Price {
    pub amount: String,
    pub currency: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pagination: Pagina
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pagina {
    total: u8,
    count: u8,
    per_page: u8,
    current_page: u8,
    total_pages: u8
}

pub fn get_price() -> Market {
    let url: String = reader::get_link(GULDEN_FILE.to_string(), String::from("price"));
    let market: String = request::get(&url);
    let market = get_market(market);
    let market = market.unwrap(); 
    market
}

fn get_market(market: String) -> Result<Market, Error> {
    let new_market: Market = serde_json::from_str(&market)?;
    Ok(new_market)
}