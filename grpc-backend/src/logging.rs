use std::fmt::Debug;
use std::time::Duration;
use tonic::codegen::http::{Request, Response};
use tower_http::trace::{OnFailure, OnRequest, OnResponse};
use tracing::Span;

#[derive(Clone)]
pub struct Logger;

impl<B: Debug> OnResponse<B> for Logger {
    fn on_response(self, response: &Response<B>, latency: Duration, _span: &Span) {
        println!("Response {:?} {}", response.status(), response.status().canonical_reason().unwrap_or_default());
        println!("- Headers: {:?}", response.headers()); // You probably don't want to log headers in production
        println!("- Latency: {:?}", latency);
        println!();
    }
}

impl<B: Debug> OnFailure<B> for Logger {
    fn on_failure(&mut self, failure_classification: B, latency: Duration, _span: &Span) {
        println!("Failure {:?}", failure_classification);
        println!("- Latency: {:?}", latency);
        println!();
    }
}

impl<B> OnRequest<B> for Logger {
    fn on_request(&mut self, request: &Request<B>, _span: &Span) {
        println!("Request {} {}", request.method().as_str(), request.uri());
        println!("- Headers: {:?}", request.headers());
        println!();
    }
}