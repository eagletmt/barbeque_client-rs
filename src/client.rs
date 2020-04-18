#[derive(Clone, Debug)]
pub struct Client {
    config: Config,
    inner: reqwest::Client,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub endpoint: url::Url,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateJobExecutionRequest<T> {
    pub application: String,
    pub job: String,
    pub queue: String,
    pub message: T,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateJobExecutionResponse {
    pub message_id: String,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            inner: Default::default(),
        }
    }

    pub fn new_with(config: Config, inner: reqwest::Client) -> Self {
        Self { config, inner }
    }

    pub async fn create_job_execution<T>(
        &self,
        request: &CreateJobExecutionRequest<T>,
    ) -> Result<CreateJobExecutionResponse, reqwest::Error>
    where
        T: serde::Serialize,
    {
        let uri = self.config.endpoint.join("v2/job_executions").unwrap();
        let response = self
            .inner
            .post(uri)
            .json(request)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(response)
    }
}
