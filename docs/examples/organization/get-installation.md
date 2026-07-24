```rust
use appwrite::Client;
use appwrite::services::Organization;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_session(""); // The user session to authenticate with

    let organization = Organization::new(&client);

    let result = organization.get_installation(
        "<INSTALLATION_ID>"
    ).await?;

    let _ = result;

    Ok(())
}
```
