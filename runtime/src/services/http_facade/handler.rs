// Waveless
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use waveless_commons::databases::DatabaseConsumer;

use crate::*;

/// TODO: add documentation.
#[derive(Clone, Debug)]
pub struct ExecuteHandler;

impl Service<RequestCx> for ExecuteHandler {
    type Response = ResponseCx;

    type Error = RequestError;

    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    /// Handles endpoints requests.
    fn call(&mut self, cx: RequestCx) -> Self::Future {
        let future: Pin<_> = Box::pin(async move {
            let RequestCx { endpoint, .. } = &cx;

            // Retrieves the endpoint's target database.
            let db_conns = endpoint.get_db_handle()?;

            // Force the endpoint to have the HTTP target.
            let ExecutionTarget::Http(http_target) = endpoint.execution_target().to_owned() else {
                unreachable!()
            };

            // Executes request.
            let Some(execute_strategy) = http_target.execution_pipeline() else {
                return Err(RequestError::Expected(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("The route doesn't have any executor defined. HINT: Go to your project's endpoints folder and check that '{}' has an executor set.", endpoint.id()).into(),
                ));
            };

            // Build the pipeline cx.
            let pipeline_cx = PipelineCx::new(cx, None);

            let (PipelineCx { response, .. }, _) = execute_strategy.executor()
                .execute(
                    pipeline_cx,
                    db_conns,
                )
                .await?;

            Ok(response.unwrap())
        }).into();

        future as Self::Future // `rust-analyzer` complains here.
    }
}
