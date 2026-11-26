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

    let result = monitors.create_smtp_monitor(
        "<MONITOR_ID>",
        "<NAME>",
        "smtp.example.com",
        587,
        Some(appwrite::enums::MonitorTlsMode::None), // optional
        Some(vec![
            Query::or(vec![
                Query::less_than("certificateDays", 14).to_string(),
                Query::greater_than("latency", 2000).to_string(),
            ]).to_string(),
        ]), // optional
        Some(60), // optional
        Some(10), // optional
        Some(300), // optional
        Some(true), // optional
        Some(vec!["owner".into()]), // optional
        Some(appwrite::enums::MonitorSeverity::Info), // optional
        Some("<ESCALATION_ID>") // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
