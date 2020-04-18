#[derive(Debug, serde::Serialize)]
struct Message {
    model_id: i64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = barbeque_client::Client::new(barbeque_client::client::Config {
        endpoint: url::Url::parse("http://localhost:3003")?,
    });
    let resp = client
        .create_job_execution(&barbeque_client::client::CreateJobExecutionRequest {
            application: "barbeque_client-rs".to_owned(),
            job: "TestJob".to_owned(),
            queue: "default".to_owned(),
            message: Message { model_id: 42 },
        })
        .await?;
    println!("Enqueued {}", resp.message_id);

    Ok(())
}
