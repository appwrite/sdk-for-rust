```rust
use appwrite::Client;
use appwrite::services::Users;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let users = Users::new(&client);

    let result = users.create_argon2_user(
        "<USER_ID>",
        "email@example.com",
        "$argon2id$v=19$m=65536,t=4,p=1$NC8xaW1CVXlIZXFSQi9FRg$0bd4pLo1ytyqvCwl5NSijpm5OIe8J6CjZ1vFemKAg+o",
        Some("<NAME>") // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
