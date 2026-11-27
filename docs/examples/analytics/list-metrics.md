```rust
use appwrite::Client;
use appwrite::services::Analytics;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.set_endpoint("https://<REGION>.cloud.appwrite.io/v1"); // Your API Endpoint
    client.set_project("<YOUR_PROJECT_ID>"); // Your project ID
    client.set_key("<YOUR_API_KEY>"); // Your secret API key

    let analytics = Analytics::new(&client);

    let result = analytics.list_metrics(
        "<PROPERTY_ID>",
        Some(vec![]), // optional
        Some(appwrite::enums::AnalyticsInterval::OneHour), // optional
        Some(vec![appwrite::enums::AnalyticsDimension::Country]), // optional
        Some("<PROP_KEY>"), // optional
        Some("<DATE_RANGE>"), // optional
        Some("2020-10-15T06:38:00.000+00:00"), // optional
        Some("2020-10-15T06:38:00.000+00:00"), // optional
        Some(1), // optional
        Some(false) // optional
    ).await?;

    let _ = result;

    Ok(())
}
```
