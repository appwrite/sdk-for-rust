```rust
use appwrite::Client;
use appwrite::services::Tokens;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let tokens = Tokens::new(&client);

    let result = tokens.create_file_token(
        "<BUCKET_ID>",
        "<FILE_ID>",
        Some("2020-10-15T06:38:00.000+00:00") // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
