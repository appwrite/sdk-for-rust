```rust
use appwrite::Client;
use appwrite::services::Oauth2;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let oauth2 = Oauth2::new(&client);

    let result = oauth2.introspect(
        "<TOKEN>",
        Some("access_token"), // optional
        Some("<CLIENT_ID>"), // optional
        Some("<CLIENT_SECRET>") // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
