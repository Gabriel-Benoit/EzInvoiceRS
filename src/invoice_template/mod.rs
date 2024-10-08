use std::{fs, io::Read, ops::Add};
use yew::{prelude::*, props};

pub mod types;
use types::*;

#[derive(Properties, PartialEq, Default)]
struct AppProps {
    data: Option<InvoiceDataJson>,
}

pub async fn render_to_str(data: &InvoiceDataJson) -> String {
    let mut result = String::new();
    let clone = data.clone();
    let renderer =
        yew::ServerRenderer::<App>::with_props(|| props!(AppProps { data: Some(clone) }));
    renderer.render_to_string(&mut result).await;
    result
}

#[function_component(App)]
fn app(props: &AppProps) -> Html {
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
        deadline,
    } = data.as_ref().unwrap();

    let Adress {
        city: e_city,
        num: e_num,
        num_suffix: e_num_suffix,
        post_code: e_post_code,
        street: e_street,
    } = &entreprise.adress;

    let Adress {
        city: b_city,
        num: b_num,
        num_suffix: b_num_suffix,
        post_code: b_post_code,
        street: b_street,
    } = &buyer.adress;

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

    let total_vat = items.iter().fold(0_f64, |acc, item| {
        if item.intra {
            acc
        } else {
            acc + item.qt * item.price_ht * (item.vat.parse::<f64>().unwrap()) / 100_f64
        }
    });

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
                                <span>{format!("{} {}{},", e_street, e_num, e_num_suffix.as_ref().unwrap_or(&String::default()) )}</span><br/>
                                <span>{format!("{} {}", e_post_code, e_city)}</span><br/>
                                <span>{ entreprise.phone.as_str() }</span><br/>
                                <span>{ entreprise.email.as_str() }</span><br/>
                                <span>{ entreprise.website.as_str() }</span>
                            </div>
                            <div>
                                <h3>{ buyer.name.as_str()}</h3>
                                <span>{format!("{} {}{},", b_street, b_num, b_num_suffix.as_ref().unwrap_or(&String::default()) )}</span><br/>
                                <span>{format!("{} {}", b_post_code, b_city)}</span><br/>
                                <p>{format!("TVA : {} ", buyer.vat_number)}</p>
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
                                                    <td>{format!("{}%",item.vat.as_str())}</td>
                                                }
                                                <td style="text-align: right">
                                                    {format!("{:1.2}", price(item)) } {"€"}
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
                                    <td><b>{"Total TVA"}</b></td>
                                    <td style="text-align: right">
                                        {format!("{:1.2}", total_vat)}{"€"}
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
                                    {format!("{:1.2}", items.iter().map(price).sum::<f64>() * due_percentage/100f64)}{"€"}
                                    </td>
                                </tr>
                                <tr>
                                    <td><b>{"Communication"}</b></td>
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
                //<img src={logo_url.as_str().to_owned()} />
            </div>
            <div id="invoice-header-info">
                <h3>{title}</h3>
                <span>{format!("Numéro de facture : {}", bill_number)}</span><br />
                <span>{format!("Date d'émission : {}", date)}</span><br />
                <span>{format!("Échéance : {}", deadline)}</span>
            </div>
        </header>
        <footer>
            <div>
                <h3>{entreprise.name.as_str()}</h3>
                <span>{format!("{} {}{},", e_street, e_num, e_num_suffix.as_ref().unwrap_or(&String::default()) )}</span><br/>
                <span>{format!("{} {}", e_post_code, e_city)}</span><br/>
                <span>{format!("TVA : {}", entreprise.vat_number.as_str())}</span>
            </div>

            <div>
                <h3>{"Contact"}</h3>
                <span>{ entreprise.name.as_str() }</span><br />
                <span>{ entreprise.phone.as_str() }</span><br />
                <span>{ entreprise.email.as_str() }</span>
            </div>

            <div>
                <h3>{"Moyen de payement"}</h3>
                <span>{ entreprise.name.as_str() }</span><br />
                <span>{ format!("IBAN : {}", entreprise.bank_account.as_str()) }</span><br />
                <span>{ format!("Banque : {}", entreprise.bank_name.as_str()) }</span>
            </div>
        </footer>
            </body>
        </html>
    )
}

pub fn get_style_str() -> Result<String, Box<dyn std::error::Error>> {
    let mut file = fs::File::open("./src/invoice_template/style.css")?;
    let mut buffer = String::new();
    buffer = buffer.add("<style>");
    file.read_to_string(&mut buffer)?;
    buffer = buffer.add("</style>");
    Ok(buffer)
}
