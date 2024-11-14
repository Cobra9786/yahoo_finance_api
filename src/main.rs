use axum::{routing::get, Json, Router, response::IntoResponse};
use serde::Serialize;
use std::{error::Error, net::SocketAddr, convert::Infallible};
use std::time::{Duration, UNIX_EPOCH};
use time::OffsetDateTime;
use time::macros::datetime;
use yahoo_finance_api as yahoo;

#[tokio::main]
async fn main() {
    // Build the Axum application with two routes
    let app = Router::new()
        .route("/latest_quote", get(get_latest_quote_handler))
        .route("/history", get(get_history_handler));

    // Define the address for the server to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    // Run the Axum server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Struct for serializing latest quote data in JSON format
#[derive(Serialize)]
struct Quote {
    timestamp: String,
    close: f64,
}

// Handler function to get the latest quote
async fn get_latest_quote_handler() -> Result<impl IntoResponse, Infallible> {
    match get_latest_quote().await {
        Ok(quote) => Ok(Json(quote).into_response()),
        Err(err) => Ok((axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()),
    }
}

// Separate function to fetch the latest quote from Yahoo Finance API
async fn get_latest_quote() -> Result<Quote, Box<dyn Error>> {
    let provider = yahoo::YahooConnector::new()?;
    let response = provider.get_latest_quotes("AAPL", "1d").await?;

    // Use `?` to handle the Result from `last_quote`
    let quote = response.last_quote()?; // handle error with `?`

    let time = OffsetDateTime::from(UNIX_EPOCH + Duration::from_secs(quote.timestamp));
    Ok(Quote {
        timestamp: time.to_string(),
        close: quote.close,
    })
}

// Struct for serializing historical quotes in JSON format
#[derive(Serialize)]
struct HistoricalQuotes {
    quotes: Vec<Quote>,
}

// Handler function to get historical quotes
async fn get_history_handler() -> Result<impl IntoResponse, Infallible> {
    match get_history().await {
        Ok(quotes) => Ok(Json(HistoricalQuotes { quotes }).into_response()),
        Err(err) => Ok((axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()),
    }
}

// Separate function to fetch historical data from Yahoo Finance API
async fn get_history() -> Result<Vec<Quote>, Box<dyn Error>> {
    let provider = yahoo::YahooConnector::new()?;
    let start = datetime!(2020-1-1 0:00:00 UTC);
    let end = datetime!(2020-1-31 23:59:59 UTC);

    let response = provider.get_quote_history("AAPL", start, end).await?;

    let quotes: Vec<Quote> = response.quotes()
        .unwrap_or_default()
        .into_iter()
        .map(|q| {
            let time = OffsetDateTime::from(UNIX_EPOCH + Duration::from_secs(q.timestamp));
            Quote {
                timestamp: time.to_string(),
                close: q.close,
            }
        })
        .collect();

    Ok(quotes)
}
