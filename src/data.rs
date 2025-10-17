#[derive(Debug)]
pub struct FarmData {
    pub title: String,
    pub phone: Option<String>,
    pub address: Option<String>,
}

impl FarmData {
    pub fn new(title: String, phone: Option<String>, address: Option<String>) -> Self {
        FarmData {
            title,
            phone,
            address
        }
    }
}