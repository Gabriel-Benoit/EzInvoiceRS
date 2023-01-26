use ez_invoice::start_server;

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Dynamic generation
    // This call blocks the current thread
    start_server().await.ok();

    Ok(())
}
