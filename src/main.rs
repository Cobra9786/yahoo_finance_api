use tokio::runtime::Runtime;
use yahoo_finance_api as yahoo;
use std::time::{Duration, UNIX_EPOCH};
use time::OffsetDateTime;
use time::macros::datetime;
use std::error::Error;

fn main() {
    // Initialize the Tokio runtime
    let rt = tokio::runtime::Runtime::new().unwrap();

    if let Err(e) = get_latest_quotes(&rt) {
        eprintln!("Error fetching latest quotes: {}", e);
    }

    if let Err(e) = get_history(&rt) {
        eprintln!("Error fetching history: {}", e);
    }
}

fn get_latest_quotes(rt: &Runtime) -> Result<(), Box<dyn Error>> {
    // Initialize the Yahoo Finance provider
    let provider = yahoo::YahooConnector::new();

    // Execute the async function within the runtime
    let response = rt.block_on(provider?.get_latest_quotes("AAPL", "1d"))?;

    // Extract the most recent quote
    let quote = response.last_quote().unwrap();

    // Convert the timestamp to a human-readable format
    let time: OffsetDateTime = OffsetDateTime::from(UNIX_EPOCH + Duration::from_secs(quote.timestamp));

    // Display the closing price
    println!("At {}, the closing price of Apple was ${}", time, quote.close);

    Ok(())
}

fn get_history(rt: &Runtime) -> Result<(), Box<dyn Error>> {
    // Initialize the Yahoo Finance provider
    let provider = yahoo::YahooConnector::new();

    // Define the start and end dates for historical data
    let start = datetime!(2020-1-1 0:00:00 UTC);
    let end = datetime!(2020-1-31 23:59:59 UTC);

    // Execute the async function within the runtime
    let resp = rt.block_on(provider?.get_quote_history("AAPL", start, end))?;

    // Extract and print the quotes
    let quotes = resp.quotes().unwrap();
    println!("Apple's quotes in January: {:?}", quotes);

    Ok(())
}
