// Waveless
// Copyright (C) 2026 Oscar Alvarez Gonzalez

use crate::*;

/// TODO: add documentation.
#[derive(Clone, Constructor, Debug)]
pub struct AuthCapture<S>
where
    S: Service<RequestCx, Response = ResponseCx, Error = RequestError>,
{
    inner: S,
}

pub struct AuthCaptureLayer;

impl<S> Layer<S> for AuthCaptureLayer
where
    S: Service<RequestCx, Response = ResponseCx, Error = RequestError>,
{
    type Service = AuthCapture<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthCapture { inner }
    }
}

impl<S> Service<RequestCx> for AuthCapture<S>
where
    S: Service<RequestCx, Response = ResponseCx, Error = RequestError> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Response: Send + 'static,
    S::Error: Send + 'static,
{
    type Response = S::Response;

    type Error = S::Error;

    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, cx: RequestCx) -> Self::Future {
        let mut inner = self.inner.to_owned();

        Box::pin(async move {
            let RequestCx { endpoint, .. } = &cx;

            match endpoint.id().as_str() {
                LOGIN_ENDPOINT_ID => LoginSvc.call(cx).await,
                SIGNUP_ENDPOINT_ID => SignUpSvc.call(cx).await,
                LOGOUT_ENDPOINT_ID => LogoutSvc.call(cx).await,
                LOGOUT_ALL_ENDPOINT_ID => LogoutSvc.call(cx).await,
                _ => inner.call(cx).await,
            }
        })
    }
}
