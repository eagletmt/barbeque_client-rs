#[derive(Debug)]
pub enum Error {
    NotFound,
    Serde(serde_json::Error),
}

#[derive(Debug)]
pub struct Job<T> {
    pub message_id: String,
    pub job: String,
    pub message: T,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::NotFound => write!(f, "Barbeque environment variables are missing"),
            Error::Serde(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

pub fn get_job<T>() -> Result<Job<T>, Error>
where
    T: serde::de::DeserializeOwned,
{
    let message_id = std::env::var("BARBEQUE_MESSAGE_ID").map_err(|_| Error::NotFound)?;
    let job = std::env::var("BARBEQUE_JOB").map_err(|_| Error::NotFound)?;
    let barbeque_message = std::env::var("BARBEQUE_MESSAGE").map_err(|_| Error::NotFound)?;
    let message = serde_json::from_str(&barbeque_message).map_err(Error::Serde)?;

    Ok(Job {
        message_id,
        job,
        message,
    })
}
