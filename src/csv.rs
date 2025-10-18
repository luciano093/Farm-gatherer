use std::error::Error;

use csv::WriterBuilder;
use serde::Serialize;

use crate::data::FarmData;

#[derive(Debug, Serialize)]
struct FarmCSVRow {
    #[serde(rename = "Type")]
    farm_type: String,
    #[serde(rename = "Farms Name")]
    farms_name: String,
    #[serde(rename = "Address")]
    address: String,
    #[serde(rename = "Point of Contact")]
    point_of_contact: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "Email")]
    email: String,
    #[serde(rename = "Phone Number")]
    phone_number: String,
    #[serde(rename = "Contacted")]
    contacted: String,
    #[serde(rename = "Interview Setup")]
    interview_setup: String,
    #[serde(rename = "Notes from meeting")]
    notes: String,
}

impl From<&FarmData> for FarmCSVRow {
    fn from(farm: &FarmData) -> Self {
        FarmCSVRow {
            farm_type: "B2B".to_string(),
            farms_name: farm.title.clone(),
            address: farm.address.clone().unwrap_or_default(),
            point_of_contact: String::new(),
            name: String::new(),
            position: String::new(),
            email: String::new(),
            phone_number: farm.phone.clone().unwrap_or_default(),
            contacted: String::new(),
            interview_setup: String::new(),
            notes: String::new(),
        }
    }
}

pub fn write_to_csv(farms: &Vec<FarmData>, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = WriterBuilder::new()
        .from_path(filename)?;
    
    for farm in farms {
        let row: FarmCSVRow = farm.into();
        wtr.serialize(row)?;
    }
    
    wtr.flush()?;
    Ok(())
}