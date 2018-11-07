#![feature(duration_as_u128)]

#[macro_use]
extern crate serde_derive;

use reqwest;
use std::collections::HashMap;

pub struct Iex {}

impl Iex {
    pub fn get_price(
        &self,
        tickers: Vec<String>,
    ) -> Result<IexTickersPrice, ()> {
        let s = tickers.join(",");

        let uri = format!("https://api.iextrading.com/1.0/stock/market/batch?symbols={}&types=price", s);

        let body = reqwest::get(uri.as_str()).unwrap().text().unwrap();

        let ret: IexTickersPrice = serde_json::from_str(&body).unwrap();

        Ok(ret)
    }
}

type IexTickersPrice = HashMap<String, IexPrice>;

#[derive(Serialize, Deserialize, Debug)]
pub struct IexPrice {
    pub price: f32,
}
