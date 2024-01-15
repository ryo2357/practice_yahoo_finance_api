// yahoo_finance_api = { "version" = "1.0", features = ["blocking"] }
use std::time::{Duration, UNIX_EPOCH};
use time::{macros::datetime, OffsetDateTime};
use yahoo_finance_api as yahoo;

fn main() {
    // get_the_latest_available_quote();
    // get_history_of_quotes_for_given_time_period();
    // get_the_history_of_quotes_for_time_range();
    search_for_a_ticker_given_a_search_string();
}

#[allow(dead_code)]
fn get_the_latest_available_quote() {
    let provider = yahoo::YahooConnector::new();
    let response = provider.get_latest_quotes("AAPL", "1d").unwrap();
    let quote = response.last_quote().unwrap();
    let time: OffsetDateTime =
        OffsetDateTime::from(UNIX_EPOCH + Duration::from_secs(quote.timestamp));
    println!("At {} quote price of Apple was {}", time, quote.close);

    // At 2024-01-12 14:30:00.0 +00:00:00 quote price of Apple was 185.9199981689453
}

#[allow(dead_code)]
fn get_history_of_quotes_for_given_time_period() {
    let provider = yahoo::YahooConnector::new();
    let start = datetime!(2020-1-1 0:00:00.00 UTC);
    let end = datetime!(2020-1-31 23:59:59.99 UTC);
    // returns historic quotes with daily interval
    let resp = provider.get_quote_history("AAPL", start, end).unwrap();
    let quotes = resp.quotes().unwrap();
    println!("Apple's quotes in January: {:?}", quotes);

    // 日足で取得してるっぽい（タイムスタンプから）
    /*
    Apple's quotes in January: [
        Quote { timestamp: 1577975400, open: 74.05999755859375, high: 75.1500015258789, low: 73.79750061035156, volume: 135480400, close: 75.0875015258789, adjclose: 73.15266418457031 },
        Quote { timestamp: 1578061800, open: 74.2874984741211, high: 75.1449966430664, low: 74.125, volume: 146322800, close: 74.35749816894531, adjclose: 72.44145202636719 }
        ・・・
    ]
     */
}
#[allow(dead_code)]
fn get_the_history_of_quotes_for_time_range() {
    let provider = yahoo::YahooConnector::new();
    let response = provider.get_quote_range("AAPL", "1d", "1mo").unwrap();
    let quotes = response.quotes().unwrap();
    println!("Apple's quotes of the last month: {:?}", quotes);
    /*
    Apple's quotes of the last month: [
        Quote { timestamp: 1702477800, open: 195.08999633789065, high: 198.0, low: 194.8500061035156, volume: 70404200, close: 197.9600067138672, adjclose: 197.9600067138672 },
        Quote { timestamp: 1702564200, open: 198.02000427246097, high: 199.6199951171875, low: 196.16000366210935, volume: 66831600, close: 198.11000061035156, adjclose: 198.11000061035156 },
                ・・・
    ]
     */
}

#[allow(dead_code)]
fn search_for_a_ticker_given_a_search_string() {
    // 検索文字列(会社名など)を指定してティッカーを検索
    let provider = yahoo::YahooConnector::new();
    let resp = provider.search_ticker("Apple").unwrap();

    // 特に使ってない
    // let mut apple_found = false;

    println!("All tickers found while searching for 'Apple':");
    for item in resp.quotes {
        println!("{}", item.symbol)
    }
}
