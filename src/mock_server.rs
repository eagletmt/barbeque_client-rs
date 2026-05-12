pub async fn start(
    addr: std::net::SocketAddr,
    command: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let listener = tokio::net::TcpListener::bind(addr).await?;
    loop {
        tokio::select! {
            accept = listener.accept() => {
                let (stream, _) = accept?;
                let command = command.clone();
                let io = hyper_util::rt::TokioIo::new(stream);
                tokio::spawn(async move {
                    let _ = hyper_util::server::conn::auto::Builder::new(hyper_util::rt::TokioExecutor::new())
                        .serve_connection(
                            io,
                            hyper::service::service_fn(move |request| {
                                let command = command.clone();
                                async move { handle(request, command).await }
                            }),
                        )
                        .await;
                });
            }
            _ = ctrl_c() => {
                break;
            }
        }
    }
    Ok(())
}

async fn ctrl_c() {
    tokio::signal::ctrl_c().await.unwrap()
}

async fn handle(
    request: hyper::Request<hyper::body::Incoming>,
    command: Vec<String>,
) -> Result<hyper::Response<http_body_util::Full<bytes::Bytes>>, hyper::Error> {
    match (request.method(), request.uri().path()) {
        (&hyper::Method::POST, "/v2/job_executions") => {
            use http_body_util::BodyExt;
            let body = request.into_body().collect().await?.to_bytes();
            let params: crate::client::CreateJobExecutionRequest<serde_json::Value> =
                serde_json::from_slice(&body).expect("Failed to parse request body");

            let message_id = uuid::Uuid::new_v4().hyphenated().to_string();

            let mut it = command.into_iter();
            let mut cmd = tokio::process::Command::new(it.next().unwrap());
            cmd.args(it)
                .env("BARBEQUE_MESSAGE_ID", &message_id)
                .env("BARBEQUE_JOB", params.job)
                .env(
                    "BARBEQUE_MESSAGE",
                    serde_json::to_string(&params.message).unwrap(),
                );
            let mut child = cmd.spawn().expect("Failed to execute command");
            tokio::spawn(async move { child.wait().await });

            let body =
                serde_json::to_vec(&crate::client::CreateJobExecutionResponse { message_id })
                    .unwrap();
            Ok(hyper::Response::new(http_body_util::Full::new(
                bytes::Bytes::from(body),
            )))
        }
        _ => Ok(hyper::Response::builder()
            .status(hyper::StatusCode::NOT_FOUND)
            .body(http_body_util::Full::new(bytes::Bytes::new()))
            .unwrap()),
    }
}
