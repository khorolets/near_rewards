use serde::{self, Deserialize};

#[derive(Deserialize, Debug)]
struct BinanceResponse {
    price: String,
}

impl BinanceResponse {
    fn get_price(&self) -> f32 {
        self.price
            .parse::<f32>()
            .unwrap()
    }
}

pub(crate) async fn binance_price() -> Result<f32, request::Error> {
    print!("Fetching NEAR-USDT price from Binance...");
    let body: BinanceResponse = request::get("https://api.binance.com/api/v3/ticker/price?symbol=NEARUSDT")
        .await?
        .json()
        .await?;
    println!(" {}", &body.get_price());
    Ok(body.get_price())
}
