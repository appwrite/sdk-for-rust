```rust
use appwrite::Client;
use appwrite::services::Account;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_session(""); // The user session to authenticate with

    let account = Account::new(&client);

    let result = account.create_email_verification(
        "https://example.com"
    ).await?;

    let _ = result;

    Ok(())
}
```
