use reqwest::blocking::{Client, Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinPrice {
    pub base: String,
    pub currency: String,
    pub amount: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinbasePrice {
    pub data: CoinPrice,
}

pub fn spot_prices(base: String, currency: String) {
    let spot_url: String = format!(
        "https://api.coinbase.com/v2/prices/{base}-{currency}/spot",
        currency = currency,
        base = base
    );

    let client: Client = Client::new();
    let resp_spot_price: Result<Response, reqwest::Error> = client.get(&spot_url).send();

    match resp_spot_price {
        Ok(parsed_spot_price) => {
            let coinprice: CoinbasePrice = parsed_spot_price.json::<CoinbasePrice>().unwrap();

            let spot_price: CoinPrice = CoinPrice {
                base: coinprice.data.base,
                currency: coinprice.data.currency,
                amount: coinprice.data.amount,
            };

            println!(
                "SPOT: {base}-{currency}: {amount}",
                base = spot_price.base,
                currency = spot_price.currency,
                amount = spot_price.amount
            );
        }
        Err(e) => println!("Err: {:?}", e),
    }
}
