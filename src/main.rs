use std::{time::Duration};

use fantoccini::{elements::Element, ClientBuilder, Locator};
use farm_gatherer::{csv::write_to_csv, data::FarmData};
use tokio::time::{sleep, timeout};

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let c = ClientBuilder::native().connect("http://localhost:49907").await.expect("failed to connect to WebDriver");

    let mut farms = Vec::new();

    c.goto("https://www.google.com/search?tbm=lcl&q=texas+farms&rflfq=1&num=10").await?;

    let clickables_count = c.find_all(Locator::Css(".rllt__details")).await?.len();

    for i in 0..clickables_count {
        let clickables = c.find_all(Locator::Css(".rllt__details")).await?;
        
        if i >= clickables.len() {
            break;
        }
        
        clickables[i].click().await?;

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

        if let Ok(detail) = c.find(Locator::Css(".xpdopen")).await {
            match handle_detals(&detail).await {
                Ok(farm_data) => farms.push(farm_data),
                Err(err) => eprintln!("{}", err),
            }
        }
        sleep(Duration::from_millis(400)).await;
        c.find(Locator::Css("div[aria-label='Close']")).await?.find(Locator::Css("span")).await?.click().await?;

        let closed = timeout(Duration::from_secs(10), async {
            loop {
                match c.find(Locator::Css(".xpdopen")).await {
                    Err(_) => break, // Element removed from DOM
                    Ok(detail) => {
                        match detail.is_displayed().await {
                            Ok(false) | Err(_) => break, // Hidden or error
                            Ok(true) => sleep(Duration::from_millis(100)).await,
                        }
                    }
                }
            }
        }).await;

        match closed {
            Ok(_) => println!("Element disappeared"),
            Err(_) => println!("Timeout: element still exists"),
        }
        sleep(Duration::from_millis(400)).await;
    }

    c.close().await?;

    write_to_csv(&farms, "farms.csv").unwrap();

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