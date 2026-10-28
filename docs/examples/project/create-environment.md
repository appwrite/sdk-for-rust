```rust
use appwrite::Client;
use appwrite::services::Project;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let project = Project::new(&client);

    let result = project.create_environment(
        "<ENVIRONMENT_ID>",
        "<NAME>",
        Some(vec![appwrite::enums::EnvironmentResource::Database, appwrite::enums::EnvironmentResource::Table, appwrite::enums::EnvironmentResource::Column, appwrite::enums::EnvironmentResource::Index, appwrite::enums::EnvironmentResource::Bucket, appwrite::enums::EnvironmentResource::Team, appwrite::enums::EnvironmentResource::Function, appwrite::enums::EnvironmentResource::EnvironmentVariable]) // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
