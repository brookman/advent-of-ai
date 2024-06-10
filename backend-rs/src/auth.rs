use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use axum::{
    http::{header::AUTHORIZATION, Request, StatusCode},
    response::{IntoResponse, Response},
};
use tower::{Layer, Service};

#[derive(Debug, Clone)]
pub struct ValidateUserOrAdminTokenLayer {
    user_token: String,
    admin_token: String,
}

impl ValidateUserOrAdminTokenLayer {
    pub fn new(user_token: &str, admin_token: &str) -> Self {
        ValidateUserOrAdminTokenLayer {
            user_token: user_token.to_string(),
            admin_token: admin_token.to_string(),
        }
    }
}

impl<S> Layer<S> for ValidateUserOrAdminTokenLayer {
    type Service = ValidateUserOrAdminToken<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ValidateUserOrAdminToken {
            inner,
            user_token: self.user_token.clone(),
            admin_token: self.admin_token.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ValidateUserOrAdminToken<S> {
    inner: S,
    user_token: String,
    admin_token: String,
}

impl<S, B> Service<Request<B>> for ValidateUserOrAdminToken<S>
where
    S: Service<Request<B>, Response = Response, Error = Infallible>,
    S::Future: Send + 'static,
    B: Send + 'static,
{
    type Response = Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let user_token = self.user_token.clone();
        let admin_token = self.admin_token.clone();

        let headers = req.headers().clone();
        let authorization_header = headers.get(AUTHORIZATION);

        let authorized = authorization_header.map_or(false, |header| {
            let header_value = header.to_str().unwrap_or("");
            header_value == format!("Bearer {}", user_token)
                || header_value == format!("Bearer {}", admin_token)
        });

        let fut = self.inner.call(req);

        Box::pin(async move {
            if authorized {
                fut.await
            } else {
                Ok(StatusCode::UNAUTHORIZED.into_response())
            }
        })
    }
}
