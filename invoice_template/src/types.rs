use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Adress {
    pub city: String,
    pub num: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "numSuffix")]
    pub num_suffix: Option<String>,
    #[serde(rename = "postCode")]
    pub post_code: String,
    pub street: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Buyer {
    pub adress: Adress,
    pub name: String,
    #[serde(rename = "vatNumber")]
    pub vat_number: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Entity {
    pub adress: Adress,
    pub name: String,
    #[serde(rename = "vatNumber")]
    pub vat_number: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Entreprise {
    pub adress: Adress,
    pub email: String,
    pub name: String,
    pub phone: String,
    #[serde(rename = "vatNumber")]
    pub vat_number: String,
    pub website: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Item {
    pub description: String,
    pub intra: bool,
    #[serde(rename = "priceHT")]
    pub price_ht: f64,
    pub qt: f64,
    pub vat: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "InvoiceData.json")]
pub struct InvoiceDataJson {
    #[serde(rename = "billNumber")]
    pub bill_number: f64,
    pub buyer: Buyer,
    pub date: String,
    #[serde(rename = "duePercentage")]
    pub due_percentage: f64,
    pub duration: f64,
    pub entreprise: Entreprise,
    pub items: Vec<Item>,
    #[serde(rename = "logoURL")]
    pub logo_url: String,
    #[serde(rename = "structuredCommunication")]
    pub structured_communication: String,
    pub title: String,
}
