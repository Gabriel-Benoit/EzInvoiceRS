use schemafy;
use serde::{Deserialize, Serialize};
use std::{fs, io::Read, ops::Add};
use stylist::{style, yew::styled_component, Style};
use yew::prelude::*;
schemafy::schemafy!(
    root: InvoiceData
    "../json-schema/schema.json");
use std::env;

const STYLE_FILE: &str = include_str!("./style.css");

#[derive(Properties, PartialEq, Default)]
pub(crate) struct AppProps {
    data: InvoiceData,
}

pub async fn render_to_str() -> Result<String, Box<dyn std::error::Error>> {
    let mut result = String::new();
    let renderer = yew::ServerRenderer::<App>::new();
    renderer.render_to_string(&mut result).await;
    Ok(result)
}

#[function_component(App)]
pub fn app() -> Html {
    //let stylesheet = Style::new(STYLE_FILE).unwrap();
    html!(
        <html>
            <head>
                <meta charset="UTF-8">
                <meta http-equiv="X-UA-Compatible" content="IE=edge">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Test</title>
            </head>
            <body>
                <div>
                    <h1>{"J'aime les licornes"}</h1>
                </div>
            </body>
        </html>
    )
}

pub fn get_style_str() -> Result<String, Box<dyn std::error::Error>> {
    let mut file = fs::File::open("./invoice_template/src/style.css").unwrap();
    let mut buffer = String::new();
    buffer = buffer.add("<style>");
    file.read_to_string(&mut buffer).unwrap();
    buffer = buffer.add("</style>");
    Ok(buffer)
}
