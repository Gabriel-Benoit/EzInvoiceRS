use std::{fs, io::Read, ops::Add};
use yew::{prelude::*, props};

pub mod types;
pub use types::*;

#[derive(Properties, PartialEq, Default)]
pub struct AppProps {
    data: Option<InvoiceDataJson>,
}

pub async fn render_to_str(data: InvoiceDataJson) -> String {
    let mut result = String::new();
    let renderer = yew::ServerRenderer::<App>::with_props(|| props!(AppProps { data: Some(data) }));
    renderer.render_to_string(&mut result).await;
    result
}

#[function_component(App)]
pub fn app(props: &AppProps) -> Html {
    let AppProps { data } = props;
    let InvoiceDataJson {
        buyer,
        date,
        due_percentage,
        duration,
        entreprise,
        items,
        logo_url,
        structured_communication,
        title,
        bill_number,
    } = data.as_ref().unwrap();
    let intra = items.iter().any(|item| item.intra);
    let price = |item: &Item| {
        item.qt
            * (item.price_ht
                + (if item.intra {
                    0_f64
                } else {
                    item.price_ht * (item.vat.parse::<f64>().unwrap()) / 100_f64
                }))
    };

    html!(
        <html>
            <head>
                <meta charset="UTF-8"/>
                <meta http-equiv="X-UA-Compatible" content="IE=edge"/>
                <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
            </head>
            <body>
            <table class="paging">
            <thead>
                <tr>
                    <td><div class="header-space"><span>{" "}</span></div></td>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>
                        <div id="invoice-from-to">
                            <div>
                                <h3>{ entreprise.name.as_str() }</h3>
                                <span>{"Entreprise ADRESS"}</span><br/>
                                <span>{ entreprise.phone.as_str() }</span><br/>
                                <span>{ entreprise.email.as_str() }</span><br/>
                                <span>{ entreprise.website.as_str() }</span>
                            </div>
                            <div>
                                <h3>{ entreprise.name.as_str()}</h3>
                                if entreprise.name != buyer.name  {
                                <span>{ buyer.name.as_str() }</span><br />
                                }
                                <span>{"Client ADRESS"}</span><br />
                                //if let Some(number) = buyer.vat_number {
                                //<p>{format!("TVA : {:?} ", number)}</p>
                                //}
                            </div>
                        </div>

                        <table class="table">
                            <thead>
                                <tr>
                                    <th>{"N°"}</th>
                                    <th>{"Description"}</th>
                                    <th>{"Qté"}</th>
                                    <th>{"Prix HT"}</th>
                                    <th>{"TVA"}</th>
                                    <th>{"Total TVAC"}</th>
                                </tr>
                            </thead>
                            <tbody>
                                {
                                    items.iter().enumerate().map(|(index, item)|{
                                        html!{
                                            <tr>
                                                <td>{format!("{:1}", index+1)}</td>
                                                <td>{item.description.as_str()}</td>
                                                <td>{item.qt}</td>
                                                <td>{format!("{:1.2}",item.price_ht)}{"€"}</td>
                                                if item.intra {
                                                    <td>{"*"}</td>
                                                } else {
                                                    <td>{item.vat.as_str()}</td>
                                                }
                                                <td style="text-align: right">
                                                    {format!("{:1.2}", price(&item)) } {"€"}
                                                </td>
                                            </tr>
                                        }
                                    }).collect::<Html>()
                                }
                                <tr>
                                    <td colspan="4" class="hidden"></td>
                                    <td><b>{"Total HT"}</b></td>
                                    <td style="text-align: right">
                                        {format!("{:1.2}",items.iter().map(|item|{
                                            item.qt * item.price_ht
                                        }).sum::<f64>())}{"€"}
                                    </td>
                                </tr>

                                <tr>
                                    <td colspan="4" class="hidden"></td>
                                    <td><b>{"Total TTC"}</b></td>
                                    <td style="text-align: right">
                                        {format!("{:1.2}", items.iter().map(price).sum::<f64>())}{"€"}
                                    </td>
                                </tr>
                            </tbody>
                        </table>

                        if intra {
                        <p>{"* Non applicable car intra-communautaire"}</p>
                        }
                        if *due_percentage > 0f64 {
                            <p><table>
                                <tr>
                                    <td><b>{"Conditions de payement"}</b></td>
                                    <td><b>{":"}</b></td>
                                    <td>
                                        {duration}{" jours"}
                                    </td>
                                </tr>
                                <tr>
                                    <td><b>{"À payer"}</b></td>
                                    <td><b>{":"}</b></td>
                                    <td>
                                    {format!("{:1.2}", items.iter().map(price).sum::<f64>() * due_percentage)}{"€"}
                                    </td>
                                </tr>
                                <tr>
                                    <td><b>{"Communication structurée"}</b></td>
                                    <td><b>{":"}</b></td>
                                    <td>
                                        {structured_communication}
                                    </td>
                                </tr>
                            </table></p>
                        }
                    </td>
                </tr>
            </tbody>
            <tfoot>
                <tr>
                    <td>
                        <div class="footer-space"><span>{" "}</span></div>
                    </td>
                </tr>
            </tfoot>
        </table>

        <header>
            <div>

            </div>
            <div id="invoice-header-info">
                <h3>{title}</h3>
                <span>{format!("Numéro de facture : {:1}", bill_number)}</span><br />
                <span>{format!("Date d'émission : {:?}", date)}</span><br />
                <span>{format!("Échéance : {:?}", duration)}</span>
            </div>
        </header>
        <footer>
            <div>
                <h3>{entreprise.name.as_str()}</h3>
                {"Entreprise ADRESS"}<br />
                <span>{format!("TVA : {:?}", entreprise.vat_number.as_str())}</span>
            </div>

            <div>
                <h3>{"Contact"}</h3>
                <span>{"Simon Loir"}</span><br />
                <span>{"+32 485 45 26 98"}</span><br />
                <span>{"contact@simonloir.be"}</span>
            </div>

            <div>
                <h3>{"Moyen de payement"}</h3>
                <span>{"Simon Loir"}</span><br />
                <span>{"IBAN : BE11 0018 6889 4148"}</span><br />
                <span>{"Banque : BNP Paribas Fortis"}</span>
            </div>
        </footer>
            </body>
        </html>
    )
}

pub fn get_style_str() -> Result<String, Box<dyn std::error::Error>> {
    let mut file = fs::File::open("./invoice_template/src/style.css")?;
    let mut buffer = String::new();
    buffer = buffer.add("<style>");
    file.read_to_string(&mut buffer)?;
    buffer = buffer.add("</style>");
    Ok(buffer)
}
