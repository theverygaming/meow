use api::api;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let instance = api::rocket().ignite().await?;

    instance.launch().await?;

    Ok(())
}
