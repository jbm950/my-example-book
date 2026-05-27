use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box< dyn std::error::Error>> {
    let token = std::fs::read_to_string("creds")?;

    let client = Client::new();
    let response = client
        .get("http://gitea.odin.orlfl.milamnet.io/api/v1/repos/search")
        .bearer_auth(token.trim())
        .send()
        .await?
        .error_for_status()?;

    println!("{}", response.text().await?);

    Ok(())
}
