use crate::types::simpledate::Simpledate;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Excelline {
    pub date: Simpledate,
    pub ticker: String,
    pub stock_price: Option<f64>,
    pub contract_type: String,
    pub strike: f64,
    pub expiry_date: Simpledate,
    pub status: String,
    pub quantity: u16,
    pub premium: f64,
}

impl Excelline {
    pub fn to_string(&self) -> String {
        let price: String = match self.stock_price {
            Some(price) => format!("{:.2}", price),
            None => "".to_string(),
        };
        let status: String = match self.status.as_str() {
            "open" => "Open".to_string(),
            "closed" => "Closed".to_string(),
            "expired" => "Expired".to_string(),
            "assigned" => "Exercised".to_string(),
            "rolled" => "Rolled".to_string(),
            "close" => "BTC".to_string(),
            "roll" => "Rolled".to_string(),
            _ => format!("Unknown: {}", self.status),
        };
        let display_type = match self.contract_type.as_str() {
            "put" => "CSP",
            "call" => "CC",
            _ => "Unknown",
        };
        format!(
            "{:02}/{:02}/{:04},{},{},{},{},{:02}/{:02}/{:04},{},{},{}",
            self.date.month,
            self.date.day,
            self.date.year,
            self.ticker,
            price,
            display_type,
            self.strike,
            self.expiry_date.month,
            self.expiry_date.day,
            self.expiry_date.year,
            status,
            self.quantity,
            (self.premium * 100.0).round() as i64
        )
    }
}
