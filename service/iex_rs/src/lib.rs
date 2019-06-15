#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[macro_use]
mod std_ext;

use reqwest;
use std::collections::HashMap;

pub struct Iex {
    pub iex_config: fin_config::IexConfig,
}

impl Iex {
    pub fn get_price(
        &self,
        tickers: Vec<String>,
    ) -> Result<IexTickersPrice, ()> {
        let s = tickers.join(",");

        let uri = format!(
            "https://cloud.iexapis.com/stable/stock/market/batch?symbols={}&types=price&token={}",
            s, self.iex_config.token
        );

        let ret: IexTickersPrice = if self.iex_config.debug {
            dbg!("using dbg source for iex price");
            let mut temp: IexTickersPrice = HashMap::new();
            let mut fake_price: f64 = 100.0;
            let mut go_up = true;
            for t in tickers {
                temp.insert(t, IexPrice { price: fake_price });
                if go_up {
                    fake_price += 15.0;
                } else {
                    fake_price -= 20.0;
                }
                go_up = !go_up;
            }
            temp
        } else {
            let body =
                reqwest::get(uri.as_str()).unwrap().text().map_err(|err| {
                    loge!(err);
                    ()
                })?;

            serde_json::from_str(&body).map_err(|err| {
                loge!(err);
                ()
            })?
        };

        Ok(ret)
    }
}

type IexTickersPrice = HashMap<String, IexPrice>;

#[derive(Serialize, Deserialize, Debug)]
pub struct IexPrice {
    pub price: f64,
}
