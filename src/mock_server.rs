pub async fn start(addr: std::net::SocketAddr, command: Vec<String>) -> Result<(), hyper::Error> {
    let make_service = hyper::service::make_service_fn(|_| {
        let command = command.clone();
        async move {
            Ok::<_, std::convert::Infallible>(hyper::service::service_fn(move |request| {
                let command = command.clone();
                async move { handle(request, command).await }
            }))
        }
    });
    let server = hyper::Server::bind(&addr)
        .serve(make_service)
        .with_graceful_shutdown(ctrl_c());
    server.await
}

async fn ctrl_c() {
    tokio::signal::ctrl_c().await.unwrap()
}

async fn handle(
    request: hyper::Request<hyper::Body>,
    command: Vec<String>,
) -> Result<hyper::Response<hyper::Body>, hyper::Error> {
    match (request.method(), request.uri().path()) {
        (&hyper::Method::POST, "/v2/job_executions") => {
            let buffer = hyper::body::aggregate(request.into_body()).await?;
            use bytes::Buf;
            let params: crate::client::CreateJobExecutionRequest<serde_json::Value> =
                serde_json::from_slice(buffer.bytes()).expect("Failed to parse request body");

            let mut message_id = vec![0; uuid::adapter::Hyphenated::LENGTH];
            uuid::Uuid::new_v4()
                .to_hyphenated()
                .encode_lower(&mut message_id);
            let message_id = String::from_utf8(message_id).unwrap();

            let mut it = command.into_iter();
            let mut cmd = std::process::Command::new(it.next().unwrap());
            cmd.args(it)
                .env("BARBEQUE_MESSAGE_ID", &message_id)
                .env("BARBEQUE_JOB", params.job)
                .env(
                    "BARBEQUE_MESSAGE",
                    serde_json::to_string(&params.message).unwrap(),
                );
            cmd.spawn().expect("Failed to execute command");

            let body =
                serde_json::to_vec(&crate::client::CreateJobExecutionResponse { message_id })
                    .unwrap();
            Ok(hyper::Response::new(hyper::Body::from(body)))
        }
        _ => Ok(hyper::Response::builder()
            .status(hyper::StatusCode::NOT_FOUND)
            .body(hyper::Body::empty())
            .unwrap()),
    }
}
