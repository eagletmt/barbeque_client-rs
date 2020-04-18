#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    barbeque_client::mock_server::start(
        "127.0.0.1:3003".parse()?,
        vec![
            "cargo".to_owned(),
            "run".to_owned(),
            "--example".to_owned(),
            "job".to_owned(),
        ],
    )
    .await?;
    Ok(())
}
