use schemafy;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
schemafy::schemafy!(
    root: InvoiceData
    "./src/json-schema/schema.json");

#[derive(Properties, PartialEq)]
pub struct MyProps {
    data: InvoiceData,
}

#[function_component]
pub fn Template(props: &MyProps) -> Html {
    html!()
}
