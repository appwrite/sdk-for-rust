```rust
use appwrite::Client;
use appwrite::services::Oauth2;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_session(""); // The user session to authenticate with
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID

    let oauth2 = Oauth2::new(&client);

    let result = oauth2.create_par(
        "<CLIENT_ID>",
        "https://example.com",
        "code",
        Some("<SCOPE>"), // optional
        Some("<STATE>"), // optional
        Some("<NONCE>"), // optional
        Some("<CODE_CHALLENGE>"), // optional
        Some("s256"), // optional
        Some("<PROMPT>"), // optional
        Some(0), // optional
        Some("<AUTHORIZATION_DETAILS>"), // optional
        Some(""), // optional
        Some("<AUDIENCE>") // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
