use std::{time::Duration};

use anyhow::Context;
use clap::{command, Parser};
use fantoccini::{elements::Element, Client, ClientBuilder, Locator};
use farm_gatherer::{csv::write_to_csv, data::FarmData};
use serde_json::json;
use tokio::time::{sleep, timeout};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// search to operate on
    #[arg(long)]
    search: String,
    #[arg(long)]
    port: u16,
    #[arg(long, num_args = 0..=1, default_missing_value = "true")]
    headless: Option<bool>,
    #[arg(long)]
    output: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let parser = Cli::parse();

    let chrome_opts = if let Some(headless) = parser.headless && headless { 
        json!({
        "args": ["--headless=new", "--disable-gpu"]
        })
    } else {
        json!({})
    };

    let mut caps = serde_json::map::Map::new();
    caps.insert("goog:chromeOptions".to_string(), chrome_opts);

    let c = ClientBuilder::native().capabilities(caps).connect(&format!("http://localhost:{}", parser.port)).await.context("Failed to connect to WebDriver")?;

    let mut farms = Vec::new();

    c.goto(&format!("https://www.google.com/search?tbm=lcl&q={}&rflfq=1&num=10", parser.search.replace(" ", "+"))).await?;

    // Wait until page loads and has the elements we need
    c.wait().for_element(Locator::Css(".rllt__details")).await?;

    let mut page = 1;
    let max_results = 50;
    let mut current_results = 0;

    let mut i = 0;
    let mut clickables = c.find_all(Locator::Css(".rllt__details")).await?;

    while current_results < max_results {
        if i >= clickables.len() {
            page += 1;

            match find_next_page_element(&c, &mut page).await {
                Ok(page_element) => page_element.click().await?,
                Err(_) => {
                    println!("Was unable to find more pages, last page found was: {}", page - 1);
                    break;
                }
            };

            i = 0;
            continue;
        }
        
        if let Err(err) = clickables[i].click().await {
            if err.is_stale_element_reference() {
                clickables = c.find_all(Locator::Css(".rllt__details")).await?;
                continue;
            } else {
                return Err(err).context("Failed to click next result.");
            }
        }

        let result = timeout(Duration::from_secs(10), async {
            loop {
                if let Ok(detail) = c.find(Locator::Css(".xpdopen")).await {
                    if let Ok(true) = detail.is_displayed().await {
                        break;
                    }
                }
                sleep(Duration::from_millis(100)).await;
            }
        })
        .await;

        match result {
            Ok(_) => println!("Element disappeared"),
            Err(_) => println!("Timeout: element still exists"),
        }

        let details_window = c.find(Locator::Css(".xpdopen")).await?;

        let farm_data = handle_detals(&details_window).await?;
        farms.push(farm_data);

        let closed = timeout(Duration::from_secs(10), async {
            loop {
                match details_window.is_displayed().await {
                    Ok(false) | Err(_) => break, // Hidden or error
                    Ok(true) => {
                        c.find(Locator::Css("div[aria-label='Close']")).await?.click().await?;
                        sleep(Duration::from_millis(100)).await;
                    },
                }
            }

            Ok::<(), anyhow::Error>(())
        })
        .await;

        match closed {
            Ok(_) => println!("Element disappeared"),
            Err(_) => println!("Timeout: element still exists"),
        }

        current_results += 1;

        i += 1;
    }

    println!("Gathered {} results.", current_results);

    c.close().await?;

    let csv_file = parser.output.as_deref().unwrap_or("data.csv");

    println!("Saving results to {}.", csv_file);
    write_to_csv(&farms, csv_file)?;

    Ok(())
}

async fn handle_detals(detail: &Element) -> Result<FarmData, fantoccini::error::CmdError> {
    let title = detail.find(Locator::Css("span")).await?.text().await?;

    let elements = detail.find_all(Locator::Css("div[role='presentation']")).await?;

    let mut phone = None;
    let mut address = None;

    for element in elements {
        let text = element.text().await?;

        if text.starts_with("Phone: ") {
            phone = Some(text.split_once("Phone: ").unwrap().1.to_string());
        } 

        if text.starts_with("Address: ") {
            address = Some(text.split_once("Address: ").unwrap().1.to_string());
        } 
    }

    let farm_data = FarmData::new(title, phone, address);

    println!("{:#?}", farm_data);

    Ok(farm_data)
}

pub async fn find_next_page_element(c: &Client, page: &mut u32) -> anyhow::Result<Element> {
    let pagination = c.find(Locator::Css("[aria-label='Local Results Pagination']")).await?;

    let page_element = pagination.find(Locator::Css(&format!("[aria-label='Page {}']", page))).await?;

    Ok(page_element)
}