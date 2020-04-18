#[derive(Debug, serde::Deserialize)]
struct Message {
    model_id: i64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let job = barbeque_client::job::get_job()?;
    let message: Message = job.message;
    println!(
        "[{}] Executing {} with {:?}",
        job.message_id, job.job, message
    );
    Ok(())
}
