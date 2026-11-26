```rust
use appwrite::Client;
use appwrite::services::Monitors;
use appwrite::query::Query;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let monitors = Monitors::new(&client);

    let result = monitors.create_usage_monitor(
        "<MONITOR_ID>",
        "<NAME>",
        vec![
            Query::greater_than("network.outbound", 5000000000).to_string(),
        ],
        Some(3600), // optional
        Some(1800), // optional
        Some(true), // optional
        Some(vec!["owner".into()]), // optional
        Some(appwrite::enums::MonitorSeverity::Info), // optional
        Some("<ESCALATION_ID>") // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
